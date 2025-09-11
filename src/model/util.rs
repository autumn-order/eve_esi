//! # EVE ESI Model Utils
//!
//! Utilties for deserializing EVE Online models

use serde::Deserialize;

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
    use crate::model::oauth2::EveJwtClaims;

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
