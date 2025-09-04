use eve_esi::error::EsiError;

use crate::oauth2::jwk::util::{
    get_jwk_internal_server_error_response, get_jwk_success_response, setup,
};

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
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock success response with expected JWT keys
///
/// # Assertions
/// - Assert that only 1 fetch attempt was made to the server
/// - Assert that the function returned the expected keys
/// - Assert that the cache has been properly updated
#[tokio::test]
async fn test_refresh_keys_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;
    let jwt_key_cache = &esi_client.jwt_key_cache;

    // Create mock response with mock keys & expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Call method under test
    // Use get_jwt_keys as entry point since refresh_jwt_keys_with_retry
    // is private
    let oauth2 = esi_client.oauth2();
    let result = oauth2.jwk().get_jwt_keys().await;

    // Assert we received only 1 expected request
    mock.assert();

    // Assert function returned expected keys
    assert!(result.is_ok());

    // Assert cache has been properly updated
    let cache = jwt_key_cache.cache.read().await;

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
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500
///
/// # Assertions
/// - Assert that 3 fetch attempts were made to the server
/// - Assert that the function returned the expected error type of
///   reqwest::Error related to status code 500.
#[tokio::test]
async fn test_refresh_keys_failure() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock response with error 500 and expecting 3 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 3);

    // Call method under test
    // Use get_jwt_keys as entry point since refresh_jwt_keys_with_retry
    // is private
    let oauth2 = esi_client.oauth2();
    let result = oauth2.jwk().get_jwt_keys().await;

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
/// - Create a basic EsiClient & mock HTTP server
/// - Configures an initial response returning an internal server error
/// - Configures a second response that successfully returns the expected keys
///
/// # Assertions
/// - Assert that 1 fetch attempt was made for each response type, an
///   error 500 response and success 200 that returned expected keys
/// - Assert that the function returned the expected keys
/// - Assert that the cache has been properly updated
#[tokio::test]
async fn test_refresh_keys_retry() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;
    let jwt_key_cache = &esi_client.jwt_key_cache;

    // Create an initial mock response with error 500 and expecting 1 request
    let mock_500 = get_jwk_internal_server_error_response(&mut mock_server, 1);

    // Create a 2nd mock response with mock keys & expecting 1 request
    let mock_200 = get_jwk_success_response(&mut mock_server, 1);

    // Call method under test
    // Use get_jwt_keys as entry point since refresh_jwt_keys_with_retry
    // is private
    let oauth2 = esi_client.oauth2();
    let result = oauth2.jwk().get_jwt_keys().await;

    // Assert we received only 1 expected request per response type
    mock_500.assert();
    mock_200.assert();

    // Assert function returned expected keys
    assert!(result.is_ok());

    // Assert cache has been properly updated
    let cache = jwt_key_cache.cache.read().await;

    assert!(*&cache.is_some())
}
