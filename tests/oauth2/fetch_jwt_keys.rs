use eve_esi::error::EsiError;
use eve_esi::model::oauth2::EveJwtKey;
use eve_esi::EsiClient;
use mockito::Server;

use super::util::create_mock_jwt_keys;

/// Tests the successful retrieval of JWT keys from a mock EVE SSO server.
///
/// # Test Setup
/// - Creates a mock server to simulate the EVE JWK endpoint
/// - Configures a mock response with expected JWT keys
/// - Points the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Verifies that the returned JWT keys match the expected keys
#[tokio::test]
async fn fetch_jwt_keys_success() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create expected JWT keys response
    let expected_keys = create_mock_jwt_keys();

    // Create mock response
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&expected_keys).unwrap())
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("UnitTest/1.0 (test@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the fetch_jwt_keys method
    let result = esi_client.oauth2().fetch_jwt_keys().await;

    // Assert
    mock.assert();
    assert!(result.is_ok());

    let jwt_keys = result.unwrap();
    assert_eq!(
        jwt_keys.skip_unresolved_json_web_keys,
        expected_keys.skip_unresolved_json_web_keys
    );
    assert_eq!(jwt_keys.keys.len(), expected_keys.keys.len());

    // Check if we have at least one key of each type
    let has_rs256 = jwt_keys
        .keys
        .iter()
        .any(|key| matches!(key, EveJwtKey::RS256 { .. }));
    let has_es256 = jwt_keys
        .keys
        .iter()
        .any(|key| matches!(key, EveJwtKey::ES256 { .. }));

    assert!(has_rs256, "Expected at least one RS256 key");
    assert!(has_es256, "Expected at least one ES256 key");
}

/// Tests error handling when retrieving JWT keys from a failing EVE SSO server.
///
/// # Test Setup
/// - Creates a mock server to simulate the EVE JWK endpoint
/// - Configures a mock response with a 500 server error
/// - Points the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Verifies that the returned error is of type ReqwestError
#[tokio::test]
async fn fetch_jwt_keys_server_error() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response with error
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("UnitTest/1.0 (test@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the fetch_jwt_keys method
    let result = esi_client.oauth2().fetch_jwt_keys().await;

    // Assert
    mock.assert();
    assert!(result.is_err());

    match result {
        Err(EsiError::ReqwestError(_)) => {} // Expected error type
        _ => panic!("Expected ReqwestError, got different error type"),
    }
}
