use eve_esi::config::EsiConfig;
use mockito::{Server, ServerGuard};
use oauth2::RequestTokenError;

use eve_esi::error::{EsiError, OAuthError};
use eve_esi::EsiClient;

use super::util::create_mock_token;

/// Provides a config with the token URL shared between tests
async fn setup() -> Result<(EsiClient, ServerGuard), EsiError> {
    let mock_server = Server::new_async().await;

    // Create config pointing token_url to the mock server
    let config = EsiConfig::builder()
        .token_url(&format!("{}/v2/oauth/token", mock_server.url()))
        .build()?;

    // Create EsiClient with custom config & oauth configured
    let client = EsiClient::builder()
        .config(config)
        .user_agent("MyApp/1.0 (contact@example.com)")
        .client_id("client_id")
        .client_secret("client_secret")
        .callback_url("http://localhost:8000/callback")
        .build()?;

    Ok((client, mock_server))
}

/// Tests the successful retrieval of an OAuth2 token
///
/// # Setup
/// - Create an EsiClient configured with OAuth2 and a mock server
/// - Configures a mock response with a successful token response
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the token response is successful
#[tokio::test]
pub async fn test_get_token_success() {
    // Create EsiClient configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await.unwrap();

    // Create mock response
    let mock_token = create_mock_token();
    let mock = mock_server
        .mock("POST", "/v2/oauth/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_token).unwrap())
        .create();

    // Create ESI client configured for OAuth2 with mock token endpoint

    // Call the get_token method
    let result = client.oauth2().get_token("authorization_code").await;

    mock.assert();
    assert!(result.is_ok());
}

/// Tests error handling when failing to retrieve an OAuth2 token
///
/// # Setup
/// - Create an EsiClient configured with OAuth2 and a mock server
/// - Configures a mock response with a bad request status code
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the error is of the [`RequestTokenError::ServerResponse`] type
#[tokio::test]
pub async fn test_get_token_error() {
    // Create EsiClient configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await.unwrap();

    // Create mock response
    let mock = mock_server
        .mock("POST", "/v2/oauth/token")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "invalid_request"}"#)
        .create();

    // Call the get_token method
    let result = client.oauth2().get_token("authorization_code").await;

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
    let config = EsiConfig::builder()
        .token_url(&format!("{}/v2/oauth/token", mock_server.url()))
        .build()
        .expect("Failed to build EsiConfig");

    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .config(config)
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
