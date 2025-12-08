use super::super::util::jwt::create_mock_token;

use mockito::{Mock, ServerGuard};

/// Returns status code 200 with a mock token
///
/// Adds a GET `/v2/oauth/token` endpoint to the mock server which returns a mock token.
///
/// # Arguments
/// - server (&mut [`mockito::ServerGuard`]): A mutable reference to a mock HTTP server which
///   will return the response
/// - expect ([`usize`]): The number of HTTP requests that should be expected
///
/// # Returns
/// - [`mockito::Mock`]: A mock used with the `.assert()` method ensure expected requests
///   were received.
pub(crate) fn get_token_success_response(server: &mut ServerGuard, expect: usize) -> Mock {
    let mock_token = create_mock_token(false);

    

    server
        .mock("POST", "/v2/oauth/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_token).unwrap())
        .expect(expect)
        .create()
}

/// Returns status code 500 internal server error
///
/// Adds a GET `/v2/oauth/token` endpoint to the mock server which returns a status code 500
/// internal server error.
///
/// # Arguments
/// - server (&mut [`mockito::ServerGuard`]): A mutable reference to a mock HTTP server which
///   will return the response
/// - expect ([`usize`]): The number of HTTP requests that should be expected
///
/// # Returns
/// - [`mockito::Mock`]: A mock used with the `.assert()` method ensure expected requests
///   were received.
pub(crate) fn get_token_bad_request_response(server: &mut ServerGuard, expect: usize) -> Mock {
    

    server
        .mock("POST", "/v2/oauth/token")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Bad request"}"#)
        .expect(expect)
        .create()
}
