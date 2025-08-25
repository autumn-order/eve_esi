use eve_esi::error::EsiError;
use eve_esi::model::oauth2::EveJwtKeys;
use eve_esi::EsiClient;
use mockito::Server;

/// Tests that JWK keys are properly fetched & cache is updated
///
/// # Test Setup
/// - Create a mock server
/// - Configures a response with expected JWT keys
/// - Points the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Verifies that fetch request was made
/// - Verifies that cache was properly updated
#[tokio::test]
async fn test_fetch_and_update_cache_success() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create expected JWT keys response
    let expected_keys = EveJwtKeys::create_mock_keys();

    // Create mock response with expected keys
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&expected_keys).unwrap())
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the fetch_and_update_cache method
    let result = esi_client.oauth2().fetch_and_update_cache().await;

    // Assert
    mock.assert();
    assert!(result.is_ok());

    // Ensure cache has been updated
    let cache = esi_client.jwt_keys_cache.read().await;
    let keys = match &*cache {
        Some((keys, timestamp)) => Some((keys.clone(), timestamp.clone())),
        None => None,
    };

    assert!(keys.is_some())
}

/// Tests that an error is properly handled when JWK fetch fails
///
/// # Test Setup
/// - Create a mock server
/// - Configures a mock response with expected JWT keys
/// - Points the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Verifies that fetch request was made
/// - Verifies that the returned error is of type ReqwestError
///   and is related to a status code 500 error.
#[tokio::test]
async fn test_fetch_and_update_cache_request_error() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response with error 500
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the fetch_and_update_cache method
    let result = esi_client.oauth2().fetch_and_update_cache().await;

    // Assert
    mock.assert();
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
        _ => panic!("Expected ReqwestError, got different error type"),
    }
}
