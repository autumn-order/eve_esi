use mockito::Server;
use oauth2::RequestTokenError;

use eve_esi::error::{EsiError, OAuthError};
use eve_esi::EsiClient;

use super::util::create_mock_token;

/// Tests the successful retrieval of an OAuth2 token
///
/// # Setup
/// - Creates a mock server to simulate the OAuth2 token endpoint
/// - Configures a mock response with a successful token response
/// - Create an [`OAuth2Config`] with the `token_url` set to the mock server
/// - Create an EsiClient using the custom [`OAuth2Config`]
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the token response is successful
#[tokio::test]
pub async fn test_get_token_success() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response
    let mock_token = create_mock_token();
    let mock = mock_server
        .mock("POST", "/v2/oauth/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_token).unwrap())
        .create();

    // Create ESI client configured for OAuth2 with mock token endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .client_id("client_id")
        .client_secret("client_secret")
        .callback_url("http://localhost:8000/callback")
        .token_url(&format!("{}/v2/oauth/token", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the get_token method
    let result = esi_client.oauth2().get_token("authorization_code").await;

    mock.assert();
    assert!(result.is_ok());
}

/// Tests error handling when failing to retrieve an OAuth2 token
///
/// # Setup
/// - Creates a mock server to simulate the OAuth2 token endpoint
/// - Configures a mock response with a bad request status code
/// - Points the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the error is of the [`RequestTokenError::ServerResponse`] type
#[tokio::test]
pub async fn test_get_token_error() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response
    let mock = mock_server
        .mock("POST", "/v2/oauth/token")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "invalid_request"}"#)
        .create();

    // Create ESI client configured for OAuth2 with mock token endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .client_id("client_id")
        .client_secret("client_secret")
        .callback_url("http://localhost:8000/callback")
        .token_url(&format!("{}/v2/oauth/token", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the get_token method
    let result = esi_client.oauth2().get_token("authorization_code").await;

    mock.assert();
    match result {
        Ok(_) => panic!("Expected an error"),
        Err(EsiError::OAuthError(OAuthError::TokenError(RequestTokenError::ServerResponse(_)))) => {
        }
        Err(err) => panic!("Expected error of type EsiError::ReqwestError: {}", err),
    }
}

/// Tests error handling when oauth client is missing
///
/// # Setup
/// - Creates a mock server to simulate the OAuth2 token endpoint
/// - Configures a mock response with a bad request status code
/// - Creates an ESI client without oauth configured
///
/// # Assertions
/// - Verifies that no request has been made to the mock server
/// - Verifies that the error is of the [`EsiError::OAuthError(OAuthError::OAuth2NotConfigured)`] type
#[tokio::test]
pub async fn test_get_token_oauth_client_missing() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response
    let mock_token = create_mock_token();
    let mock = mock_server
        .mock("POST", "/v2/oauth/token")
        .with_status(200)
        .expect(0) // Expect no calls
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_token).unwrap())
        .create();

    // Create ESI client without OAuth2 config & with mock token endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .token_url(&format!("{}/v2/oauth/token", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the get_token method
    let result = esi_client.oauth2().get_token("authorization_code").await;

    // Assert
    mock.assert();
    match result {
        Ok(_) => panic!("Expected an error"),
        Err(EsiError::OAuthError(OAuthError::OAuth2NotConfigured)) => {}
        Err(_) => {
            panic!("Expected error of type EsiError::OAuthError(OAuthError::OAuth2NotConfigured)")
        }
    }
}
