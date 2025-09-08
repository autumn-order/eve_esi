use super::super::util::jwt::create_mock_token_keys;

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
pub(crate) fn get_jwk_success_response(server: &mut ServerGuard, expect: usize) -> Mock {
    let mock_keys = create_mock_token_keys();

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
pub(crate) fn get_jwk_internal_server_error_response(
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
