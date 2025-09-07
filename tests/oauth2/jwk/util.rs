use eve_esi::model::oauth2::{EveJwtKey, EveJwtKeys};

use mockito::{Mock, ServerGuard};

/// Returns status code 200 with mock jwk keys
///
/// Adds a GET `/oauth/jwks` endpoint to the mock server which returns a set of mock
/// JWK keys as expected from EVE Online's OAuth2 API.
///
/// # Arguments
/// - server `&mut [`mocktio::ServerGuard``: A mutable reference to a mock HTTP server which
///   will return the response
/// - expect `usize`: The number of HTTP requests that should be expected
///
/// # Returns
/// - [`mockito::Mock`]: A mock used with the `.assert()` method ensure expected requests
///   were received.
pub(super) fn get_jwk_success_response(server: &mut ServerGuard, expect: usize) -> Mock {
    let mock_keys = EveJwtKeys::create_mock_keys();

    let mock = server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_keys).unwrap())
        .expect(expect)
        .create();

    mock
}

/// Returns status code 500 internal server error
///
/// Adds a GET `/oauth/jwks` endpoint to the mock server which returns a status code 500
/// internal server error.
///
/// # Arguments
/// - server `&mut[`mocktio::ServerGuard`]`: A mutable reference to a mock HTTP server which
///   will return the response
/// - expect `usize`: The number of HTTP requests that should be expected
///
/// # Returns
/// - [`mockito::Mock`]: A mock used with the `.assert()` method ensure expected requests
///   were received.
pub(super) fn get_jwk_internal_server_error_response(
    server: &mut ServerGuard,
    expect: usize,
) -> Mock {
    let mock = server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(expect)
        .create();

    mock
}

/// Helper function to create mock JWT keys for testing cache refresh
pub fn create_mock_jwt_keys_alternative() -> EveJwtKeys {
    EveJwtKeys {
        skip_unresolved_json_web_keys: true, // Different from the other mock
        keys: vec![
            EveJwtKey::RS256 {
                e: "AQAB".to_string(),
                kid: "JWT-Signature-Key-3".to_string(), // Different kid
                kty: "RSA".to_string(),
                n: "vX1oo9bD4DQBZa4qP0W0HZK2sNM3JRj3n5UZ1qJ9WqFpOvG43UqKVeSoK5jIIZ9OyTQCJFN3WUuGfFWuXIQUQ-YQgNzBu9NrGfSqZjgS5j3xgxWTQ2aaCQC8CyNDwIPvHFsB3nI9SPjVJxwoKaceTLMV98_5IMydZYpDXWv8qahA1wIbjrwFkDm6uKxRkUwRWjOcK3GVtYjBnmrcaQK5_6gbfBgOt2kkE3QRFNZdUSkvU6M0DTQj4JpJ8zUFRB0Z3HVarJ_LXzlRkXAjggItTYINijMNzcROLfLdQA9U0q-JiU8EhRkD9LJXSQgQXE5hXRQwGjSH_QJWIoQcdQ".to_string(), // Different n
                r#use: "sig".to_string(),
            },
            EveJwtKey::ES256 {
                crv: "P-256".to_string(),
                kid: "JWT-Signature-Key-4".to_string(), // Different kid
                kty: "EC".to_string(),
                r#use: "sig".to_string(),
                x: "F0KvrJXqZJ8avKyHx3EZpGbIHwYZPiBgmX0oRbbO9_c".to_string(), // Different x
                y: "NluE_RjRJbxNCFnG9oqB_3KJq0bLiQJGlXrfEiT6Oig".to_string(), // Different y
            },
        ],
    }
}
