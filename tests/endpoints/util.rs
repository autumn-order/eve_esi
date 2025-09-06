use eve_esi::{config::EsiConfig, EsiClient};
use mockito::{Server, ServerGuard};

/// Utility function to create initial test setup for all ESI endpoint integration tests
///
/// # Setup
/// - Create a mock server using the [`mockito`] crate to handle HTTP requests at mock endpoints
/// - Create an [`EsiConfig`] with the `esi_url` set to the mock server
/// - Create an EsiClient using the custom [`EsiConfig`]
///
/// # Returns
/// A tuple containing:
/// - [`eve_esi::EsiClient`]: A basic EsiClient with jwk_url set to the mock server
/// - [`mockito::ServerGuard`]: A mock server for handling http requests for test purposes
pub(super) async fn setup() -> (EsiClient, ServerGuard) {
    // Setup mock server
    let mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create a config pointing the ESI URL to the mock server
    let config = EsiConfig::builder()
        .esi_url(&mock_server_url)
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
