use mockito::{Mock, ServerGuard};

use crate::model::oauth2::{EveJwtKey, EveJwtKeys};

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
    let mock_keys = create_mock_keys();

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

/// Utility function to create a set of mock JWT keys
///
/// Creates an RS256 & an ES256 variant of EveJwtKey within an
/// EveJwtKeys struct to mock the data returned from EVE's
/// OAuth2 JWT key API for the purposes of testing
///
/// # Returns
/// - [`EveJwtKeys`]: which contains Vec<[`EveJwtKey::RS256`], [`EveJwtKey::ES256`] &
///   the skip_unresolved_json_web_keys field set to `false`
pub fn create_mock_keys() -> EveJwtKeys {
    EveJwtKeys {
            skip_unresolved_json_web_keys: false,
            keys: vec![
                EveJwtKey::RS256 {
                    e: "AQAB".to_string(),
                    kid: "JWT-Signature-Key-1".to_string(),
                    kty: "RSA".to_string(),
                    n: "nehPQ7FQ1YK-leKyIg-aACZaT-DbTL5V1XpXghtLX_bEC-fwxhdE_4yQKDF6cA-V4c-5kh8wMZbfYw5xxgM9DynhHGNLbZpmfmbQZ3X-ZUwpZ4ARuYKKM8vGXaUxOH7rKjF4SWjbaPZR8wZO9TcLRUvuRjBppP_8JM3DTCfs0nD-r3J_5uUvXWGR_bFQ1s-Ucn3_QxQqR_D5wDJRx5ZiKIxja2IZg4PGNp5WdBBY-KwmyMxzYQvKWLlcjv5FRJVupKWcJgJ0uLgqBYLiKJFja3RSlQnK1ph__gIEFMnjXEQJhEQb5JdV9H8JaP_MxQi2-8SdCG4ZpAQwTZoIgQ".to_string(),
                    r#use: "sig".to_string(),
                },
                EveJwtKey::ES256 {
                    crv: "P-256".to_string(),
                    kid: "JWT-Signature-Key-2".to_string(),
                    kty: "EC".to_string(),
                    r#use: "sig".to_string(),
                    x: "ITcDYJ8WVpDO4QtZ169xXUt7GB1Y6-oMKIwJ3nK1tFU".to_string(),
                    y: "ZAJr0f4V2Eu7xBgLMgQBdJ2DZ2mp8JykOhX4XgU_UEY".to_string(),
                },
            ],
        }
}
