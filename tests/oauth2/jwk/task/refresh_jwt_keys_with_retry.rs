use eve_esi::error::EsiError;
use eve_esi::model::oauth2::EveJwtKeys;
use eve_esi::EsiClient;
use mockito::Server;

// TODO: When JWT config default constants are made configurable,
// adjust these integration tests to change exponential backoff
// period to 1ms to reduce test time.

/// Validates successful refresh on first attempt
///
/// Attempts to refresh & update JWT key cache from a mock server
/// representing EVE Online OAuth2 API.
/// Only 1 fetch attempt should be made as it will
/// be a success on the first try.
///
/// # Test Setup
/// - Create a mock server
/// - Configures a response with expected JWT keys
/// - Point the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Assert that only 1 fetch attempt was made to the server
/// - Assert that the function returned the expected keys
/// - Assert that the cache has been properly updated
#[tokio::test]
async fn test_refresh_keys_success() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create expected JWT keys response
    let expected_keys = EveJwtKeys::create_mock_keys();

    // Create mock response with expected keys & expecting
    // only 1 request
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&expected_keys).unwrap())
        .expect(1)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call method under test
    // Use get_jwt_keys as entry point since refresh_jwt_keys_with_retry
    // is private
    let oauth2 = esi_client.oauth2();
    let result = oauth2.get_jwt_keys().await;

    // Assert we received only 1 expected request
    mock.assert();

    // Assert function returned expected keys
    assert!(result.is_ok());
    let jwt_keys = result.unwrap();
    assert_eq!(jwt_keys.keys.len(), expected_keys.keys.len());

    // Assert cache has been properly updated
    let cache = esi_client.jwt_key_cache.read().await;

    assert!(*&cache.is_some())
}

/// Validates error handling should all attempts fail
///
/// Attempts to refresh & update JWT key cache from a mock server
/// representing the EVE Online OAuth2 API.
/// All attempts will fail due to the mock server returning
/// error code 500 on each attempt. The function should retry
/// for a total of 3 attempts before returning an error.
///
/// # Test Setup
/// - Create a mock server
/// - Configure a response returning error code 500 for each request
/// - Point the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Assert that 3 fetch attempts were made to the server
/// - Assert that the function returned the expected error type of
///   reqwest::Error related to status code 500.
#[tokio::test]
async fn test_refresh_keys_failure() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response returning status 500 and
    // expecting 3 requests
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(3)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call method under test
    // Use get_jwt_keys as entry point since refresh_jwt_keys_with_retry
    // is private
    let oauth2 = esi_client.oauth2();
    let result = oauth2.get_jwt_keys().await;

    // Assert we received only 3 expected requests
    mock.assert();

    // Assert function returned expected error
    assert!(result.is_err());
    match result {
        Err(EsiError::ReqwestError(err)) => {
            // Ensure reqwest error is of type 500 server error
            assert!(err.is_status());
            assert_eq!(
                err.status(),
                Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
            )
        }
        _ => panic!("Expected EsiError::ReqwestError, got different error type"),
    }
}

/// Validates successful refresh after 2 attempts
///
/// Attempts to refresh & update JWT key cache from a mock server
/// representing the EVE Online OAuth2 API.
/// First attempt will fail due receiving status code 500,
/// second attempt will succeed returning the expected keys
///
/// # Test Setup
/// - Create a mock server
/// - Configures an initial response returning an internal server error
/// - Configures a second response that successfully returns the expected keys
/// - Point the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Assert that 1 fetch attempt was made for each response type, an
///   error 500 response and success 200 that returned expected keys
/// - Assert that the function returned the expected keys
/// - Assert that the cache has been properly updated
#[tokio::test]
async fn test_refresh_keys_retry() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create expected JWT keys response
    let expected_keys = EveJwtKeys::create_mock_keys();

    // Create initial mock response with error 500
    let mock_500 = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(1)
        .create();

    // Create second mock response with 200 success and
    // expected keys
    let mock_200 = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&expected_keys).unwrap())
        .expect(1)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call method under test
    // Use get_jwt_keys as entry point since refresh_jwt_keys_with_retry
    // is private
    let oauth2 = esi_client.oauth2();
    let result = oauth2.get_jwt_keys().await;

    // Assert we received only 1 expected request per response type
    mock_500.assert();
    mock_200.assert();

    // Assert function returned expected keys
    assert!(result.is_ok());
    let jwt_keys = result.unwrap();
    assert_eq!(jwt_keys.keys.len(), expected_keys.keys.len());

    // Assert cache has been properly updated
    let cache = esi_client.jwt_key_cache.read().await;

    assert!(*&cache.is_some())
}
