use eve_esi::error::EsiError;
use eve_esi::model::oauth2::EveJwtKey;
use eve_esi::EsiClient;

use crate::oauth2::jwk::util::{
    get_jwk_internal_server_error_response, get_jwk_success_response, setup,
};

/// Tests the successful retrieval of JWT keys from a mock EVE SSO server.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock success response with expected JWT keys
///
/// # Assertions
/// - Assert that 1 fetch request was made to mock server
/// - Assert result is Ok
/// - Assert response returned expected mock keys
/// - Assert we have at least 1 key of expected type
#[tokio::test]
async fn fetch_jwt_keys_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock response with mock keys & expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Call the fetch_jwt_keys method
    let result = esi_client.oauth2().fetch_jwt_keys().await;

    // Assert mock server received 1 expected fetch request
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Assert response returned expected mock keys
    let jwt_keys = result.unwrap();

    assert_eq!(jwt_keys.skip_unresolved_json_web_keys, false);
    assert_eq!(jwt_keys.keys.len(), 2);

    // Check if we have at least one key of each type
    let has_rs256 = jwt_keys
        .keys
        .iter()
        .any(|key| matches!(key, EveJwtKey::RS256 { .. }));
    let has_es256 = jwt_keys
        .keys
        .iter()
        .any(|key| matches!(key, EveJwtKey::ES256 { .. }));

    // Assert we have 1 of each expected key type
    assert!(has_rs256, "Expected at least one RS256 key");
    assert!(has_es256, "Expected at least one ES256 key");
}

/// Tests error handling when retrieving JWT keys from a failing EVE SSO server.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500 and expecting 1 request
///
/// # Assertions
/// - Assert mock server received 1 expected request
/// - Assert result is error
/// - Assert error is of type [`reqwest::StatusCode::INTERNAL_SERVER_ERROR`]
#[tokio::test]
async fn fetch_jwt_keys_server_error() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock response with error 500 and expecting 1 request
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 1);

    // Call the fetch_jwt_keys method
    let result = esi_client.oauth2().fetch_jwt_keys().await;

    // Assert mock server received 1 expected fetch request
    mock.assert();

    // Assert result is error
    assert!(result.is_err());

    match result {
        Err(EsiError::ReqwestError(err)) => {
            // Assert error is reqwest error of type 500 internal server error
            assert!(err.is_status());
            assert_eq!(
                err.status(),
                Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
            )
        }
        _ => panic!("Expected ReqwestError, got different error type"),
    }
}

/// Tests error handling when a network error occurs requesting JWT keys.
///
/// # Test Setup
/// - Create an ESI client with a JWK url set to an invalid endpoint
///
/// # Assertions
/// - Assert result is error
/// - Assert error is related to a reqwest connection issue
#[tokio::test]
async fn fetch_jwt_keys_network_error() {
    // Create ESI client with invalid mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("http://127.0.0.1"))
        .build()
        .expect("Failed to build EsiClient");

    // Call the fetch_jwt_keys method
    let result = esi_client.oauth2().fetch_jwt_keys().await;

    // Assert result is error
    assert!(result.is_err());

    match result {
        Err(EsiError::ReqwestError(err)) => {
            // Assert reqwest error is related to a connection issue
            assert!(err.is_connect())
        }
        _ => panic!("Expected ReqwestError, got different error type"),
    }
}

/// Tests error handling when server returns an invalid response body
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock success response with an unexpected response body
///
/// # Assertions
/// - Assert mock server received 1 expected fetch request
/// - Assert fetch result is error
/// - Assert error is of type [`reqwest::error::Kind::Decode`]
#[tokio::test]
async fn fetch_jwt_keys_parse_error() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock success response with unexpected body
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Unexpected response body"}"#)
        .create();

    // Call the fetch_jwt_keys method
    let result = esi_client.oauth2().fetch_jwt_keys().await;

    // Assert mock server received 1 expected fetch request
    mock.assert();

    // Assert result is error
    assert!(result.is_err());

    match result {
        Err(EsiError::ReqwestError(err)) => {
            // Assert reqwest error is related to decoding the body
            assert!(err.is_decode())
        }
        _ => panic!("Expected ReqwestError, got different error type"),
    }
}
