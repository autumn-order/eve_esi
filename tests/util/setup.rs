use std::time::Duration;

use mockito::{Server, ServerGuard};

use crate::constant::TEST_CLIENT_ID;

/// Utility function to create initial test setup for all jwk integration tests
///
/// # Setup
/// - Create a mock server using the [`mockito`] crate to handle HTTP requests at mock endpoints
/// - Create an [`Config`] with the `jwk_url` set to the mock server & reduced wait times
/// - Create an Client using the custom [`Config`]
///
/// # Returns
/// A tuple containing:
/// - [`eve_esi::Client`]: A basic Client with jwk_url set to the mock server
/// - [`mockito::ServerGuard`]: A mock server for handling http requests for test purposes
pub async fn integration_test_setup() -> (eve_esi::Client, ServerGuard) {
    // Setup mock server
    let mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create a config with mock server JWK URL & reduced wait times
    let config = eve_esi::Config::builder()
        // Set endpoints to mock server
        .esi_url(&mock_server_url)
        .token_url(&format!("{}/v2/oauth/token", mock_server.url()))
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        // Set exponential backoff between refresh retries to 1 millisecond
        .jwk_refresh_backoff(Duration::from_millis(1))
        // Set timeout to 1 second when waiting for another thread to refresh
        .jwk_refresh_timeout(Duration::from_secs(1))
        // Reduce cache lifetime & background refresh threshold for get_jwt_key tests
        // Set to milliseconds to ensure expiry calculations are precise
        .jwk_cache_ttl(Duration::from_millis(900))
        .jwk_background_refresh_threshold(50) // 50% expiry for background refresh, 450 milliseconds
        .build()
        .expect("Failed to build Config");

    // Create ESI client with OAuth2 & the custom config
    let esi_client = eve_esi::Client::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .client_id(TEST_CLIENT_ID)
        .client_secret("client_secret")
        .callback_url("http://localhost:8000/callback")
        .config(config)
        .build()
        .expect("Failed to build Client");

    (esi_client, mock_server)
}
