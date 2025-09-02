use eve_esi::error::EsiError;

use crate::oauth2::jwk::util::{get_jwk_internal_server_error_response, get_jwk_success_response};

use super::super::util::setup;

/// Tests that JWK keys are properly fetched & cache is updated
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock success response with expected JWT keys
///
/// # Assertions
/// - Assert that fetch request was made
/// - Assert that cache was properly updated
#[tokio::test]
async fn test_fetch_and_update_cache_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;
    let jwt_key_cache = &esi_client.jwt_key_cache;

    // Create mock response with mock keys & expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Call the fetch_and_update_cache method
    let result = esi_client.oauth2().fetch_and_update_cache().await;

    // Assert mock server received 1 expected fetch request
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Ensure cache has been updated
    let cache = jwt_key_cache.cache.read().await;
    let keys = match &*cache {
        Some((keys, timestamp)) => Some((keys.clone(), timestamp.clone())),
        None => None,
    };

    assert!(keys.is_some())
}

/// Tests that an error is properly handled when JWK fetch fails
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500
///
/// # Assertions
/// - Assert that fetch request was made
/// - Assert that the returned error is of type ReqwestError
///   and is related to a status code 500 error.
#[tokio::test]
async fn test_fetch_and_update_cache_request_error() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock response with error 500 and expecting 1 request
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 1);

    // Call the fetch_and_update_cache method
    let result = esi_client.oauth2().fetch_and_update_cache().await;

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
