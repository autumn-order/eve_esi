use eve_esi::EsiClient;
use eve_esi::{model::oauth2::EveJwtKeys, oauth2::jwk::JwtKeyCache};
use mockito::{Mock, Server, ServerGuard};

/// Utility function to create initial test setup for all jwk integration tests
///
/// # Setup
/// - Create a mock server using the [`mockito`] crate to handle HTTP requests at mock endpoints
/// - Create an [`OAuth2Config`] with the `jwk_url` set to the mock server
/// - Create an EsiClient using the custom [`OAuth2Config`]
///
/// # Returns
/// A tuple containing:
/// - [`eve_esi::EsiClient`]: A basic EsiClient with jwk_url set to the mock server
/// - [`mockito::ServerGuard`]: A mock server for handling http requests for test purposes
pub(super) async fn setup() -> (EsiClient, ServerGuard) {
    // Setup mock server
    let mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create an OAuth2 cache using the mock JWK endpoint & reduced wait times for testing
    let cache = JwtKeyCache::builder()
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        // Set expoential backoff between refresh retries to 1 millisecond
        .refresh_backoff(1)
        // Set timeout to 1 second when waiting for another thread to refresh
        .refresh_timeout(1)
        .build()
        .expect("Failed to build JWT key cache");

    // Create ESI client with the custom cache config
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwt_key_cache(cache)
        .build()
        .expect("Failed to build EsiClient");

    (esi_client, mock_server)
}

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
