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
//! ## EVE Online OAuth2 Documentation
//! - <https://developers.eveonline.com/docs/services/sso/>

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Represents the claims in an EVE Online JWT access token
///
/// This struct contains the standard JWT claims as well as EVE Online specific
/// claims that are used to identify the character and other information.
///
/// # ESI Documentation
/// - <https://developers.eveonline.com/docs/services/sso/#validating-jwt-tokens>
///
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
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at time (Unix timestamp)
    pub iat: i64,
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
    /// Utility function to create a mock of EveJwtClaims
    pub fn mock() -> Self {
        // Get current unix timestamp
        let unix_timstamp_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

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
            exp: unix_timstamp_now + 900, // Valid for 15 minutes
            iat: unix_timstamp_now,
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
pub(super) fn deserialize_scp<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
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

#[cfg(test)]
mod deserialize_scp_tests {
    use super::EveJwtClaims;

    /// Test direct deserialization of JSON into EveJwtClaims with null scp field
    #[test]
    fn test_deserialize_scp_no_scopes() {
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
            // scp field does not exist when no scopes are requested
            "name": "Test Character",
            "owner": "123456789",
            "azp": "client_id"
        });

        // Deserialize directly from JSON
        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        // Verify the scp field was deserialized to an empty vector
        assert!(
            claims.scp.is_empty(),
            "Expected empty vector for null scopes, got: {:?}",
            claims.scp
        );
    }

    /// Test direct deserialization of JSON into EveJwtClaims with a single string scp field
    #[test]
    fn test_deserialize_scp_single_scope() {
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

        // Deserialize directly from JSON
        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        // Verify the scp field was deserialized to a vector with a single element
        assert_eq!(
            claims.scp.len(),
            1,
            "Expected vector with 1 element for single scope"
        );
        assert_eq!(claims.scp[0], "publicData", "Expected 'publicData' scope");
    }

    /// Test direct deserialization of JSON into EveJwtClaims with multiple scopes in an array
    #[test]
    fn test_deserialize_scp_multiple_scopes() {
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

        // Deserialize directly from JSON
        let claims: EveJwtClaims =
            serde_json::from_value(json_data).expect("Failed to deserialize claims");

        // Verify the scp field was deserialized to a vector with multiple elements
        assert_eq!(
            claims.scp.len(),
            2,
            "Expected vector with 2 elements for multiple scopes"
        );
        assert_eq!(claims.scp[0], "publicData");
        assert_eq!(claims.scp[1], "esi-characters.read_agents_research.v1");
    }
}
