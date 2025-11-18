use std::time::Duration;

use chrono::Utc;
use mockito::{Server, ServerGuard};

use crate::config::Config;
use crate::model::oauth2::EveJwtClaims;
use crate::Client;

/// Utility function to create initial test setup for all HTTP-related unit tests
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
pub(crate) async fn setup() -> (Client, ServerGuard) {
    // Setup mock server
    let mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create a config with mock server JWK URL & reduced wait times
    let config = Config::builder()
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        // Set exponential backoff between refresh retries to 1 millisecond
        .jwk_refresh_backoff(Duration::from_millis(1))
        // Set timeout to 1 second when waiting for another thread to refresh
        .jwk_refresh_timeout(Duration::from_secs(1))
        .build()
        .expect("Failed to build Config");

    // Create ESI client with the custom config
    let esi_client = Client::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .config(config)
        .build()
        .expect("Failed to build Client");

    (esi_client, mock_server)
}

/// Utility function to create a mock of EveJwtClaims
pub fn create_mock_jwt_claims() -> EveJwtClaims {
    let expires_in_fifteen_minutes = Utc::now() + chrono::Duration::seconds(900);
    let created_now = Utc::now();

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
        exp: expires_in_fifteen_minutes,
        iat: created_now,
        scp: vec![],
        name: "Test Character".to_string(),
        owner: "123456789".to_string(),
        azp: "client_id".to_string(),
    }
}
