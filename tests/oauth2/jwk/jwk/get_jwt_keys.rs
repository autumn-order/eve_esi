use std::time::Instant;

use eve_esi::model::oauth2::EveJwtKeys;

use crate::oauth2::jwk::util::{
    get_jwk_internal_server_error_response, get_jwk_success_response, setup,
};

use super::super::mock::create_mock_jwt_keys_alternative;

/// Tests that get_jwt_keys returns cached keys when they are not expired.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response expeting 0 requests
/// - Pre-populates the cache with valid keys
///
/// # Assertions
/// - Assert that no fetch request is made
/// - Assert result is Ok
#[tokio::test]
async fn get_jwt_keys_valid_cache() {
    let (esi_client, mut mock_server) = setup().await;

    // Create a mock response expecting 0 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

    // Create cached JWT keys
    let cached_keys = EveJwtKeys::create_mock_keys();

    // Pre-populate the cache
    {
        let mut cache = esi_client.jwt_key_cache.write().await;
        *cache = Some((cached_keys.clone(), Instant::now()));
    }

    // Call the method under test
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert no fetch requests were made
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());
}

/// Tests that get_jwt_keys fetches fresh keys when the cache is expired.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock success response with expected JWT keys
/// - Pre-populates the cache with expired keys
///
/// # Assertions
/// - Assert that 1 fetch request was made to mock server
/// - Asserts result is Ok
/// - Assert expected keys were returned
/// - Assert key cache is not empty
/// - Assert keys in cache are no longer expired
#[tokio::test]
async fn get_jwt_keys_expired_cache() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock response with mock keys & expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Pre-populate the cache with expired keys
    {
        let mut cache = esi_client.jwt_key_cache.write().await;

        let timestamp = std::time::Instant::now() - std::time::Duration::from_secs(3600);
        let mock_keys = create_mock_jwt_keys_alternative();

        *cache = Some((mock_keys, timestamp));
    }

    // Call the method under test
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert mock server received 1 expected fetch request
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Assert expected keys were returned
    let jwt_keys = result.unwrap();
    assert_eq!(jwt_keys.keys.len(), 2);

    // Cache should be updated with new keys
    {
        let cache = esi_client.jwt_key_cache.read().await;

        // Assert cache is not empty
        assert!(cache.is_some());
        let (_, timestamp) = cache.as_ref().unwrap();

        // Assert keys in cache are no longer expired
        // By default keys expire after 3600 seconds
        let expired = std::time::Instant::now() - std::time::Duration::from_secs(3600);

        assert!(timestamp > &expired)
    }
}

/// Tests that get_jwt_keys fetches fresh keys when the cache is empty.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock success response with expected JWT keys
/// - JWT key cache will be empty by default
///
/// # Assertions
/// - Assert mock server received 1 expected fetch request
/// - Assert result is ok
/// - Assert JWT key cache is no longer empty
#[tokio::test]
async fn get_jwt_keys_empty_cache() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock response with mock keys & expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Call the method under test
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert mock server received 1 expected fetch request
    mock.assert();

    // Assert result is ok
    assert!(result.is_ok());

    // Cache should be updated with new keys
    {
        let cache = esi_client.jwt_key_cache.read().await;

        // Assert cache is no longer empty
        assert!(cache.is_some());
    }
}
