use oauth2::RequestTokenError;

use crate::{
    oauth2::token::util::{get_token_bad_request_response, get_token_success_response},
    util::setup,
};

/// Tests the successful retrieval of an OAuth2 token
///
/// # Setup
/// - Create Client configured with OAuth2 & mock server
/// - Create mock response with 200 success response & mock token
///
/// # Assertions
/// - Assert only 1 fetch request was made
/// - Assert result is ok
#[tokio::test]
pub async fn test_get_token_success() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create mock response with 200 success response & mock token
    let mock = get_token_success_response(&mut mock_server, 1);

    // Call the get_token method
    let result = client.oauth2().get_token("authorization_code").await;

    // Assert only 1 fetch request was made
    mock.assert();

    // Assert result is ok
    assert!(result.is_ok());
}

/// Tests error handling when failing to retrieve an OAuth2 token
///
/// # Setup
/// - Create Client configured with OAuth2 & mock server
/// - Create mock response returning a 400 bad request
///
/// # Assertions
/// - Assert only 1 fetch request was made
/// - Assert result is err
/// - Assert error is of type RequestTokenError::ServerResponse
#[tokio::test]
pub async fn test_get_token_error() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create mock response returning a 400 bad request
    let mock = get_token_bad_request_response(&mut mock_server, 1);

    // Call the get_token method
    let result = client.oauth2().get_token("authorization_code").await;

    // Assert only 1 fetch request was made
    mock.assert();

    // Assert result is err
    assert!(result.is_err());

    // Assert error is of type RequestTokenError::ServerResponse
    assert!(matches!(
        result,
        Err(eve_esi::Error::OAuthError(
            eve_esi::OAuthError::RequestTokenError(RequestTokenError::ServerResponse(_))
        ))
    ));
}

/// Tests error handling when oauth client is missing
///
/// # Setup
/// - Create ESI client without OAuth2 config & with mock token endpoint
/// - Create mock response which shouldn't be fetched
/// - Creates an ESI client without oauth configured
///
/// # Assertions
/// - Assert no fetch request was made
/// - Assert result is error
/// - Assert error is of type OAuthError::OAuth2NotConfigured
#[tokio::test]
pub async fn test_get_token_oauth_client_missing() {
    let (_, mut mock_server) = setup().await;

    // Create ESI client without OAuth2 config & with mock token endpoint
    let config = eve_esi::Config::builder()
        .token_url(&format!("{}/v2/oauth/token", mock_server.url()))
        .build()
        .expect("Failed to build Config");

    let esi_client = eve_esi::Client::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .config(config)
        .build()
        .expect("Failed to build Client");

    // Create mock response which shouldn't be fetched
    let mock = get_token_bad_request_response(&mut mock_server, 0);

    // Call the get_token method
    let result = esi_client.oauth2().get_token("authorization_code").await;

    // Assert no fetch request was made
    mock.assert();

    // Assert result is error
    assert!(result.is_err());

    // Assert error is of type OAuthError::OAuth2NotConfigured
    assert!(matches!(
        result,
        Err(eve_esi::Error::OAuthError(
            eve_esi::OAuthError::OAuth2NotConfigured
        ))
    ))
}
