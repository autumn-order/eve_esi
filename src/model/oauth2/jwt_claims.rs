//! # EVE Online OAuth2 JWT Claims Model
//!
//! Provides the [`EveJwtClaims`] struct to represent the claims of a JWT
//! token returned from EVE Online's OAuth2 API after login. These claims
//! are returned after using the [crate::oauth2::OAuth2Api::validate_token]
//! method.
//!
//! For usage of OAuth2 in the `eve_esi` crate, please see the [`crate::oauth2`]
//! module documentation.
//!
//! ## Methods
//! The [`EveJwtClaims`] struct has the following utility methods:
//! - [`EveJwtClaims::character_id`]: Utility function to parse the [`EveJwtClaims::sub`] field into a character ID
//! - [`EveJwtClaims::is_expired`]: Utility function to check token claims to see if it is expired
//!
/// ## EVE Online OAuth2 Documentation
/// - <https://developers.eveonline.com/docs/services/sso/#validating-jwt-tokens>
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{Error, OAuthError};

/// Represents the claims in an EVE Online JWT access token
///
/// This struct contains the standard JWT claims as well as EVE Online specific
/// claims that are used to identify the character and other information.
///
/// # EVE Online OAuth2 Documentation
/// - <https://developers.eveonline.com/docs/services/sso/#validating-jwt-tokens>
#[derive(Debug, Serialize, Deserialize)]
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
            // Token is not yet expired
            let time_remaining = self.exp - now;
            let message = format!(
                "Checked token for expiration, token for character ID {} is not yet expired, expiration in {}s",
                character_id,
                time_remaining.num_seconds()
            );
            log::debug!("{}", message);

            return false;
        }

        // Token is expired
        let message = format!(
            "Checked token for expiration, token for character ID {} is expired",
            character_id
        );
        log::debug!("{}", message);

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
                let message = format!(
                    "Token for character ID {} is missing scope: {}",
                    character_id, expected_scope
                );
                log::debug!("{}", message);

                return false;
            }
        }

        // All expected scopes were found
        let message = format!(
            "Token for character ID {} has all expected scopes",
            character_id
        );
        log::debug!("{}", message);

        true
    }

    /// Utility function to create a mock of EveJwtClaims
    pub fn mock() -> Self {
        // Create JWT mock claims matching what EVE Online would return
        EveJwtClaims {
            // ESI SSO docs defines 2 different JWT issuers but typically only returns 1 of them at a time
            // The default defines 2 but for tests we'll define 1 to ensure validation works
            iss: "https://login.eveonline.com".to_string(),
            sub: "CHARACTER:EVE:123456789".to_string(),
            aud: vec!["client_id".to_string(), "EVE Online".to_string()],
            jti: "abc123def456".to_string(),
            kid: "JWT-Signature-Key-1".to_string(),
            tenant: "tranquility".to_string(),
            region: "world".to_string(),
            exp: Utc::now() + Duration::seconds(900), // Valid for 15 minutes
            iat: Utc::now(),
            scp: vec![
                "publicData".to_string(),
                "esi-characters.read_agents_research.v1".to_string(),
            ],
            name: "Test Character".to_string(),
            owner: "123456789".to_string(),
            azp: "client_id".to_string(),
        }
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

/// Deserializes i64 unix timestamp common for JWTs into DateTime<Utc>
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
        Ok(Utc
            .timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?)
    }
}

#[cfg(test)]
mod claims_character_id_tests {
    use crate::{model::oauth2::EveJwtClaims, Error, OAuthError};

    /// Success when `sub` field is properly formatted
    #[test]
    fn test_claims_character_id_success() {
        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.sub = "CHARACTER:EVE:123456789".to_string();

        let result = mock_claims.character_id();

        assert!(result.is_ok());

        let character_id = result.unwrap();
        assert_eq!(character_id, 123456789);
    }

    /// Error when sub field is not segmented into 3 parts by ":"
    #[test]
    fn test_claims_character_id_segment_error() {
        let mut mock_claims = EveJwtClaims::mock();
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
        let mut mock_claims = EveJwtClaims::mock();
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

    use crate::model::oauth2::EveJwtClaims;

    /// Ensures that when token is not expired, function returns false
    #[tokio::test]
    pub async fn test_is_expired_false() {
        let mock_claims = EveJwtClaims::mock();

        let result = mock_claims.is_expired();

        assert_eq!(result, false);
    }

    /// Ensures that when token is expired, function returns true
    #[tokio::test]
    async fn test_is_expired_true() {
        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.exp = Utc::now() - Duration::seconds(60); // Expired 1 minute ago
        mock_claims.iat = Utc::now() - Duration::seconds(960); // Created 16 minutes ago

        let result = mock_claims.is_expired();

        assert_eq!(result, true);
    }
}

#[cfg(test)]
mod has_scopes_tests {
    use crate::model::oauth2::EveJwtClaims;

    /// Test that function returns true since all scopes are present
    #[test]
    fn test_has_scopes_true() {
        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = vec!["publicData".to_string()];

        let expected_scopes = vec!["publicData".to_string()];
        let result = mock_claims.has_scopes(&expected_scopes);

        assert_eq!(result, true);
    }

    /// Test that function returns false due to missing scopes
    #[test]
    fn test_has_scopes_false() {
        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = vec!["".to_string()];

        let expected_scopes = vec!["publicData".to_string()];
        let result = mock_claims.has_scopes(&expected_scopes);

        assert_eq!(result, false);
    }
}

#[cfg(test)]
mod deserialize_scp_tests {
    use super::EveJwtClaims;

    /// Test direct deserialization of JSON into EveJwtClaims with null scp field
    #[test]
    fn test_deserialize_scp_null() {
        let json_data = serde_json::json!({
            "iss": "https://login.eveonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": 1,
            "iat": 1,
            "scp": null,
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        });

        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        // Verify the scp field was deserialized to an empty vector
        assert!(claims.scp.is_empty());
    }

    /// Test direct deserialization of JSON into EveJwtClaims with a single string scp field
    #[test]
    fn test_deserialize_scp_single_string() {
        let json_data = serde_json::json!({
            "iss": "https://login.eveonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": 1,
            "iat": 1,
            "scp": "publicData",  // single string scope
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        });

        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        assert_eq!(claims.scp.len(), 1);
        assert_eq!(claims.scp[0], "publicData");
    }

    /// Test direct deserialization of JSON into EveJwtClaims with multiple scopes in an array
    #[test]
    fn test_deserialize_scp_string_array() {
        let json_data = serde_json::json!({
            "iss": "https://login.eveoonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": 1,
            "iat": 1,
            "scp": ["publicData", "esi-characters.read_agents_research.v1"],  // Array of scopes
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        });

        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        assert_eq!(claims.scp.len(), 2);
        assert_eq!(claims.scp[0], "publicData");
        assert_eq!(claims.scp[1], "esi-characters.read_agents_research.v1");
    }

    /// Error when `scp` field is not null, string, or array
    #[test]
    fn test_deserialize_scp_not_string() {
        let json_data = serde_json::json!({
            "iss": "https://login.eveonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": 1,
            "iat": 1,
            "scp": 488,
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        });

        let result = serde_json::from_value::<EveJwtClaims>(json_data);

        assert!(result.is_err());

        assert!(matches!(result,
            Err(err) if err.to_string().contains("Expected null, string, or string array for `scp` field")
        ));
    }

    /// Error when `scp` field is an array but not of strings
    #[test]
    fn test_deserialize_scp_not_string_array() {
        let json_data = serde_json::json!({
            "iss": "https://login.eveonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": 1,
            "iat": 1,
            "scp": [9,9,9],
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        });

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
        let iat = Utc::now();

        serde_json::json!({
            "iss": "https://login.eveonline.com",
            "sub": "CHARACTER:EVE:123456789",
            "aud": ["client_id".to_string(), "EVE Online"],
            "jti": "abc123def456",
            "kid": "JWT-Signature-Key-1",
            "tenant": "tranquility",
            "region": "world",
            "exp": exp_value.into(),
            "iat": iat.timestamp(),
            "scp": [],
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        })
    }

    /// Error due to invalid i64 unix timestamp
    #[test]
    fn test_jwt_timestamp_format_success() {
        // Valid for 15 minutes
        let exp = Utc::now() + Duration::seconds(900);

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
