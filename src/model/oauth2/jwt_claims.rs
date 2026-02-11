//! # EVE Online OAuth2 JWT Claims Model
//!
//! Provides the [`EveJwtClaims`] struct to represent the claims of a JWT
//! token returned from EVE Online's OAuth2 API after login. These claims
//! are returned after using the [crate::oauth2::OAuth2Endpoints::validate_token]
//! method.
//!
//! For usage of OAuth2 in the `eve_esi` crate, please see the [`crate::oauth2`]
//! module documentation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Error, OAuthError};

/// Represents the claims in an EVE Online JWT access token
///
/// This struct contains the standard JWT claims as well as EVE Online specific
/// claims that are used to identify the character and other information.
///
/// # EVE Online OAuth2 Documentation
/// - <https://developers.eveonline.com/docs/services/sso/#validating-jwt-tokens>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EveJwtClaims {
    // There are two possible issuers but only 1 will be present at a time
    // See `constant.rs`, `DEFAULT_JWT_ISSUERS` for possible issuers.
    /// The issuer of the JWT token (EVE Online's login service URL)
    pub iss: String,
    /// ID for the authenticated user (Example: "CHARACTER:EVE:2114794365")
    pub sub: String,
    /// Audience the JWT token is intended for (your client_id, EVE Online)
    pub aud: Vec<String>,
    /// JWT token ID, a unique identifier for this specific token
    pub jti: String,
    /// Key ID identifying which key was used to sign this JWT
    pub kid: String,
    /// The EVE Online server the key is for (tranquility)
    pub tenant: String,
    /// The region from which the token was issued (world)
    pub region: String,
    /// Expiration time
    #[serde(with = "jwt_timestamp_format")]
    pub exp: DateTime<Utc>,
    /// Issued at time
    #[serde(with = "jwt_timestamp_format")]
    pub iat: DateTime<Utc>,
    // This field behaves oddly when deserializing due to:
    // - 0 scopes requested: `scp` field won't exist on claims body
    // - 1 scope requested: Field exists as String
    // - 2 scopes requested: Field exists as an array of Strings
    //
    // As a result, we need to use a custom deserializer function to handle this
    /// The scopes granted by this token
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_scp")]
    pub scp: Vec<String>,
    /// The character's name
    pub name: String,
    /// The character's ID
    pub owner: String,
    /// Client ID
    pub azp: String,
}

impl EveJwtClaims {
    /// Utility function to parse the [`EveJwtClaims::sub`] field into a character ID
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - [`i64`]: The Character ID present in the [`EveJwtClaims::sub`] field
    /// - [`Error`]: An error if the [`EveJwtClaims::sub`] field can't be parsed into an [`i64`].
    ///   This shouldn't occur unless EVE Online changes the format of the field.
    pub fn character_id(&self) -> Result<i64, Error> {
        // Split the ID from the text
        let segments = self.sub.split(':').collect::<Vec<&str>>();

        // Return error if `sub` field does not match expected format
        // This is necessary otherwise the function will panic
        let segments_len = segments.len();
        if segments_len != 3 {
            let message = format!(
                "The `sub` field segment length is {} but the expected length is 2",
                segments_len,
            );

            log::error!("{}", message);

            return Err(Error::OAuthError(OAuthError::CharacterIdParseError(
                message,
            )));
        }

        match segments[2].parse::<i64>() {
            Ok(character_id) => Ok(character_id),
            Err(err) => {
                let message = format!("Failed to parse `sub` field to i64 due to error: {}", err);

                log::error!("{}", message);

                Err(Error::OAuthError(OAuthError::CharacterIdParseError(
                    message,
                )))
            }
        }
    }

    /// Utility function to check token claims to see if it is expired
    ///
    /// If your token is expired then a request to an authenticated ESI route will return an error. It is ideal to
    /// stop the request from happening within your application to not incur ESI error limits.
    ///
    /// # Returns
    /// - `bool`: Indicating whether or not token is expired
    pub fn is_expired(&self) -> bool {
        let character_id = self.character_id().unwrap_or(0);

        let now = Utc::now();
        let token_expiration = self.exp;

        if now < token_expiration {
            let time_remaining = self.exp - now;

            log::debug!(
                "Checked token for expiration, token for character ID {} is not yet expired, expiration in {}s",
                character_id,
                time_remaining.num_seconds()
            );

            return false;
        }

        let time_remaining = now - self.exp;
        log::debug!(
            "Checked token for expiration, token for character ID {} is expired, expired {}s ago",
            character_id,
            time_remaining.num_seconds()
        );

        true
    }

    /// Utility function to check if claims has provided scopes
    ///
    /// If your token is missing the scopes required for an authenticated ESI route your request will return
    /// an error. It is ideal to stop the request from happening within your application to not incur ESI error limits.
    ///
    /// You can use the [`crate::ScopeBuilder`] with this method, calling [`crate::ScopeBuilder::build`] will
    /// convert it into a `Vec<String>` as required by this method's arguments.
    ///
    /// # Arguments
    /// - `scopes` (`Vec<String>`): An array of scope strings validated against the `claims.scp` field to ensure it contains
    ///   all provided scopes.
    ///
    /// # Returns
    /// - `bool`: Indicating if all scopes provided are present.
    pub fn has_scopes(&self, scopes: &Vec<String>) -> bool {
        // Set character_id for logging to 0 if `sub` field can't be parsed to id
        let character_id = self.character_id().unwrap_or(0);

        // Check if `claims.scp` contains all expected scopes
        for expected_scope in scopes {
            if !self.scp.iter().any(|scope| scope == expected_scope) {
                // One of the expected scopes is missing
                log::debug!(
                    "Token for character ID {} is missing scope: {}",
                    character_id,
                    expected_scope
                );

                return false;
            }
        }

        // All expected scopes were found
        log::debug!(
            "Token for character ID {} has all expected scopes",
            character_id
        );

        true
    }
}

/// Custom deserializer for the `scp` field in JWT claims
///
/// Handles the three possible formats from EVE Online's JWT tokens:
/// - 0 scopes requested: `scp` field won't exist on claims body
/// - 1 scope requested: Field exists as String
/// - 2 scopes requested: Field exists as an array of Strings
fn deserialize_scp<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    // Use Option to handle the case where the field is missing
    let opt_value = Option::<Value>::deserialize(deserializer)?;

    match opt_value {
        // Missing field
        None => Ok(Vec::new()),

        // Field exists
        Some(value) => match value {
            // Single scope as a string
            Value::String(s) => Ok(vec![s]),

            // Multiple scopes as an array
            Value::Array(arr) => {
                let mut scopes = Vec::with_capacity(arr.len());
                for item in arr {
                    if let Value::String(s) = item {
                        scopes.push(s);
                    } else {
                        return Err(Error::custom("Expected string array for scopes"));
                    }
                }
                Ok(scopes)
            }

            // Any other type
            _ => Err(Error::custom(
                "Expected null, string, or string array for `scp` field",
            )),
        },
    }
}

/// Deserializes i64 unix timestamp common for JWTs into `DateTime<Utc>`
mod jwt_timestamp_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Converts DateTime<Utc> into an i64 unix timestamp for serializing token claims primarily used in tests
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    /// Converts i64 unix timestamp into a DateTime<Utc> for the EveJwtClaims struct
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;

        Utc.timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))
    }
}

#[cfg(test)]
mod claims_character_id_tests {
    use crate::{tests::util::create_mock_jwt_claims, Error, OAuthError};

    /// Success when `sub` field is properly formatted
    #[test]
    fn test_claims_character_id_success() {
        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.sub = "CHARACTER:EVE:123456789".to_string();

        let result = mock_claims.character_id();

        assert!(result.is_ok());

        let character_id = result.unwrap();
        assert_eq!(character_id, 123456789);
    }

    /// Error when sub field is not segmented into 3 parts by ":"
    #[test]
    fn test_claims_character_id_segment_error() {
        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.sub = "not_segmented".to_string();

        let result = mock_claims.character_id();

        assert!(result.is_err());

        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::CharacterIdParseError(_)))
        ))
    }

    /// Error when `sub` field is properly segmented but expected ID is not a number
    #[test]
    fn test_claims_character_id_i64_parse_error() {
        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.sub = "CHARACTER:EVE:not_a_number".to_string();

        let result = mock_claims.character_id();

        assert!(result.is_err());

        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::CharacterIdParseError(_)))
        ))
    }
}

#[cfg(test)]
mod is_expired_tests {

    use chrono::{Duration, Utc};

    use crate::tests::util::create_mock_jwt_claims;

    /// Ensures that when token is not expired, function returns false
    #[tokio::test]
    pub async fn test_is_expired_false() {
        let mock_claims = create_mock_jwt_claims();

        let result = mock_claims.is_expired();

        assert!(!result);
    }

    /// Ensures that when token is expired, function returns true
    #[tokio::test]
    async fn test_is_expired_true() {
        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.exp = Utc::now() - Duration::seconds(60); // Expired 1 minute ago
        mock_claims.iat = Utc::now() - Duration::seconds(960); // Created 16 minutes ago

        let result = mock_claims.is_expired();

        assert!(result);
    }
}

#[cfg(test)]
mod has_scopes_tests {
    use crate::tests::util::create_mock_jwt_claims;

    /// Test that function returns true since all scopes are present
    #[test]
    fn test_has_scopes_true() {
        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.scp = vec!["publicData".to_string()];

        let expected_scopes = vec!["publicData".to_string()];
        let result = mock_claims.has_scopes(&expected_scopes);

        assert!(result);
    }

    /// Test that function returns false due to missing scopes
    #[test]
    fn test_has_scopes_false() {
        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.scp = vec!["".to_string()];

        let expected_scopes = vec!["publicData".to_string()];
        let result = mock_claims.has_scopes(&expected_scopes);

        assert!(!result);
    }
}

#[cfg(test)]
mod deserialize_scp_tests {
    use std::time::Duration;

    use chrono::Utc;
    use serde_json::Value;

    use super::EveJwtClaims;

    /// Helper function to create mock JSON data for tests with configurable `scp` field
    fn create_mock_json<T>(scp: T) -> serde_json::Value
    where
        T: serde::Serialize,
    {
        let expires_in_fifteen_minutes = Utc::now() + Duration::from_secs(900);
        let created_now = Utc::now();

        serde_json::json!({
            "iss": "https://login.eveonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": expires_in_fifteen_minutes.timestamp(),
            "iat": created_now.timestamp(),
            "scp": scp,
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        })
    }

    /// Test direct deserialization of JSON into EveJwtClaims with null scp field
    #[test]
    fn test_deserialize_scp_null() {
        let json_data = create_mock_json(Value::Null);

        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        // Verify the scp field was deserialized to an empty vector
        assert!(claims.scp.is_empty());
    }

    /// Test direct deserialization of JSON into EveJwtClaims with a single string scp field
    #[test]
    fn test_deserialize_scp_single_string() {
        let json_data = create_mock_json("publicData");

        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        assert_eq!(claims.scp.len(), 1);
        assert_eq!(claims.scp[0], "publicData");
    }

    /// Test direct deserialization of JSON into EveJwtClaims with multiple scopes in an array
    #[test]
    fn test_deserialize_scp_string_array() {
        let json_data =
            create_mock_json(vec!["publicData", "esi-characters.read_agents_research.v1"]);

        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        assert_eq!(claims.scp.len(), 2);
        assert_eq!(claims.scp[0], "publicData");
        assert_eq!(claims.scp[1], "esi-characters.read_agents_research.v1");
    }

    /// Error when `scp` field is not null, string, or array
    #[test]
    fn test_deserialize_scp_not_string() {
        let json_data = create_mock_json(488);

        let result = serde_json::from_value::<EveJwtClaims>(json_data);

        assert!(result.is_err());

        assert!(matches!(result,
            Err(err) if err.to_string().contains("Expected null, string, or string array for `scp` field")
        ));
    }

    /// Error when `scp` field is an array but not of strings
    #[test]
    fn test_deserialize_scp_not_string_array() {
        let json_data = create_mock_json(vec![9, 9, 9]);

        let result = serde_json::from_value::<EveJwtClaims>(json_data);

        assert!(result.is_err());

        assert!(matches!(result,
            Err(err) if err.to_string().contains("Expected string array for scopes")
        ));
    }
}

#[cfg(test)]
mod test_jwt_timestamp_format {
    use super::EveJwtClaims;

    use chrono::{Duration, Utc};

    /// Helper function to create JSON test data with a customizable exp field
    fn create_test_jwt_json(exp_value: impl Into<serde_json::Value>) -> serde_json::Value {
        let created_now = Utc::now();

        serde_json::json!({
            "iss": "https://login.eveonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": exp_value.into(),
            "iat": created_now.timestamp(),
            "scp": [],
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        })
    }

    /// Error due to invalid i64 unix timestamp
    #[test]
    fn test_jwt_timestamp_format_success() {
        // Valid for 20 minutes
        let exp = Utc::now() + Duration::seconds(1200);

        let json_data = create_test_jwt_json(exp.timestamp());
        let result = serde_json::from_value::<EveJwtClaims>(json_data);

        assert!(result.is_ok());
    }

    /// Error due to invalid i64 unix timestamp
    #[test]
    fn test_jwt_timestamp_format_invalid_timestamp() {
        // Falls outside of DateTime<Utc> representable range
        let json_data = create_test_jwt_json(i64::MAX);
        let result = serde_json::from_value::<EveJwtClaims>(json_data);

        assert!(result.is_err());

        assert!(matches!(result,
            Err(err) if err.to_string().contains("Invalid timestamp")
        ));
    }
}
