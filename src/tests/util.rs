use std::time::Duration;

use mockito::{Server, ServerGuard};

use crate::config::EsiConfig;
use crate::EsiClient;

/// Utility function to create initial test setup for all HTTP-related unit tests
///
/// # Setup
/// - Create a mock server using the [`mockito`] crate to handle HTTP requests at mock endpoints
/// - Create an [`EsiConfig`] with the `jwk_url` set to the mock server & reduced wait times
/// - Create an EsiClient using the custom [`EsiConfig`]
///
/// # Returns
/// A tuple containing:
/// - [`eve_esi::EsiClient`]: A basic EsiClient with jwk_url set to the mock server
/// - [`mockito::ServerGuard`]: A mock server for handling http requests for test purposes
pub(crate) async fn setup() -> (EsiClient, ServerGuard) {
    // Setup mock server
    let mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create a config with mock server JWK URL & reduced wait times
    let config = EsiConfig::builder()
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        // Set exponential backoff between refresh retries to 1 millisecond
        .jwk_refresh_backoff(Duration::from_millis(1))
        // Set timeout to 1 second when waiting for another thread to refresh
        .jwk_refresh_timeout(Duration::from_secs(1))
        .build()
        .expect("Failed to build EsiConfig");

    // Create ESI client with the custom config
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .config(config)
        .build()
        .expect("Failed to build EsiClient");

    (esi_client, mock_server)
}
