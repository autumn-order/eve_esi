use std::time::Duration;

use crate::oauth2::util::jwk_response::{
    get_jwk_internal_server_error_response, get_jwk_success_response,
};
use crate::util::setup;

/// Tests that get_jwt_keys returns cached keys when they are not expired.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configure a mock response expecting only 1 request for initial cache population
/// - Pre-populate the cache with valid keys
///
/// # Assertions
/// - Assert cache was initially populated without issues
/// - Assert that only 1 fetch request was made
/// - Assert result is Ok
#[tokio::test]
async fn get_jwt_keys_valid_cache() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create a mock response expecting 1 request for initial cache population
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Pre-populate the cache
    let result = esi_client.oauth2().jwk().fetch_and_update_cache().await;

    // Assert cache was initially populated without issues
    assert!(result.is_ok());

    // Call the method under test
    let result = esi_client.oauth2().jwk().get_jwt_keys().await;

    // Assert only 1 fetch request was made
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());
}

/// Tests that get_jwt_keys fetches fresh keys when the cache is expired.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Create mock success response expecting 2 requests:
/// - Pre-populate the cache
/// - Wait for the cache to expire (2 seconds)
///
/// # Assertions
/// - Assert cache was initially populated without issues
/// - Assert mock server received 2 expected fetch requests
/// - Assert result is Ok
#[tokio::test]
async fn get_jwt_keys_expired_cache() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock success response expecting 2 requests:
    // - Pre-populating the cache
    // - Refreshing the expired cache
    let mock = get_jwk_success_response(&mut mock_server, 2);

    // Pre-populate the cache
    let result = esi_client.oauth2().jwk().fetch_and_update_cache().await;

    // Assert cache was initially populated without issues
    assert!(result.is_ok());

    // Wait for cache to expire
    // For testing, the cache expiry is set to 2 seconds
    tokio::time::sleep(std::time::Duration::from_millis(2100)).await;

    // Call the method under test
    let result = esi_client.oauth2().jwk().get_jwt_keys().await;

    // Assert mock server received 2 expected fetch requests
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());
}

/// Tests that get_jwt_keys fetches fresh keys when the cache is empty.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock response with mock keys & expecting 1 request
///
/// # Assertions
/// - Assert mock server received 1 expected fetch request
/// - Assert result is ok
#[tokio::test]
async fn get_jwt_keys_empty_cache() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock response with mock keys & expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Call the method under test
    let result = esi_client.oauth2().jwk().get_jwt_keys().await;

    // Assert mock server received 1 expected fetch request
    mock.assert();

    // Assert result is ok
    assert!(result.is_ok());
}

/// An error will be returned due to refresh cooldown still being active
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create a mock response expecting 3 requests
///
/// # Assertions
/// - Assert initial attempt resulted in an error
/// - Assert mock server received 3 expected fetch request from initial attempt
/// - Assert result is error
/// - Assert error is of the OAuthError:JwtKeyRefreshCooldown variant
#[tokio::test]
async fn get_jwt_keys_refresh_cooldown() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create a mock response expecting 3 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 3);

    // Make an initial attempt to update the cache which which will fail
    let result = esi_client.oauth2().jwk().get_jwt_keys().await;

    // Assert initial attempt resulted in an error
    assert!(result.is_err());

    // Call the method under test
    let result = esi_client.oauth2().jwk().get_jwt_keys().await;

    // Assert mock server received 3 expected fetch request from initial attempt
    mock.assert();

    // Assert result is error
    assert!(result.is_err());
    match result {
        // Assert error is of the OAuthError:JwtKeyRefreshCooldown variant
        Err(eve_esi::Error::OAuthError(eve_esi::OAuthError::JwtKeyRefreshCooldown(_))) => {}
        err => panic!(
            "Expected OAuthError::JwtKeyRefreshCooldown error, recieved: {:#?}.",
            err
        ),
    }
}

/// Tests that background refresh is triggered when cache is past proactive refresh threshold
///
/// When the JWT key cache is a certain % to expiry determined by the background_refresh_threshold
/// set on the EsiConfig for EsiClient, a background refresh is triggered to refresh keys
/// proactively.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create a mock response expecting 2 requests
/// - Pre-populate the cache
/// - Wait a moment for cache to reach background refresh threshold (1100ms)
///
/// # Assertions
/// - Assert cache was initially populated without issues
/// - Assert 2 fetch requests were made, the pre-populate & background refresh
/// - Assert result is Ok
#[tokio::test]
async fn get_jwt_keys_background_refresh() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create a mock response expecting 2 requests:
    // - Pre-populate the cache
    // - Background refresh
    let mock = get_jwk_success_response(&mut mock_server, 2);

    // Assert cache was initially populated without issues
    let result = esi_client.oauth2().jwk().fetch_and_update_cache().await;

    // Assert cache was successfully updated
    assert!(result.is_ok());

    // Wait a moment for cache to reach background refresh threshold
    // For testing, we set cache expiry to 2 seconds & threshold to 50%
    tokio::time::sleep(Duration::from_millis(1100)).await;

    // Call the method under test
    let result = esi_client.oauth2().jwk().get_jwt_keys().await;

    // Wait for background refresh to run
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Assert 2 fetch requests were made, pre-populating the cache & background refresh
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());
}

/// Tests only 1 request is made when get_jwt_keys is called concurrently
///
/// Spawns 3 tasks that all attempt to get JWT keys from an empty cache which would
/// prompt them to attempt to refresh the keys. They'll attempt to acquire a refresh lock,
/// whichever one acquires the lock performs a refresh while the other 2 tasks
/// will wait for the refresh to complete and then return the keys from the cache.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create a mock response expecting 1 request
/// - Spawn 3 concurrent tasks that all call get_jwt_keys
/// - Wait for all tasks to complete
///
/// # Assertions
/// - Assert only 1 fetch request was made
/// - Assert all tasks got valid results
#[tokio::test]
async fn get_jwt_keys_concurrency() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create a mock response expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Create shared client reference
    let esi_client = std::sync::Arc::new(esi_client);

    // Create 3 tasks to call get_jwt_keys concurrently
    let client1 = esi_client.clone();
    let task1 = tokio::spawn(async move { client1.oauth2().jwk().get_jwt_keys().await });

    let client2 = esi_client.clone();
    let task2 = tokio::spawn(async move { client2.oauth2().jwk().get_jwt_keys().await });

    let client3 = esi_client.clone();
    let task3 = tokio::spawn(async move { client3.oauth2().jwk().get_jwt_keys().await });

    // Wait for all tasks to complete
    let result1 = task1.await.expect("Task 1 panicked");
    let result2 = task2.await.expect("Task 2 panicked");
    let result3 = task3.await.expect("Task 3 panicked");

    // Assert only 1 fetch request was made
    mock.assert();

    // Assert all tasks got valid results
    assert!(result1.is_ok(), "Task 1 failed: {:?}", result1.err());
    assert!(result2.is_ok(), "Task 2 failed: {:?}", result2.err());
    assert!(result3.is_ok(), "Task 3 failed: {:?}", result3.err());
}
