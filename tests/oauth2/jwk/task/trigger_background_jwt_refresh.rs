use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;

use eve_esi::model::oauth2::EveJwtKeys;

use crate::oauth2::jwk::util::{
    get_jwk_internal_server_error_response, get_jwk_success_response, setup,
};

// Note: Need 100ms delay to accurately wait for background refresh to properly notify
// of completion. Maybe a refactor is necessary to avoid this wait for testing purposes?

/// Tests the background refresh if keys are approaching expiry
///
/// When the JWT keys present in cache are approaching expiry,
/// being past 80% expired by default, the function should
/// trigger a background refresh.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock success response with expected JWT keys
/// - Populate cache with keys past 80% expired to trigger background refresh
/// - Call the function & wait for a refresh complete notification
///   or timeout if background refresh takes too long.
///
/// # Assertions
/// - Assert a notification has been received of completed refresh
/// - Assert 1 request has been made to mock server
/// - Assert refresh lock has been released
/// - Assert cache has been properly updated with keys not nearing expiration
#[tokio::test]
async fn test_background_refresh_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (mut esi_client, mut mock_server) = setup().await;

    // Create mock response with mock keys & expecting 1 request
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Populate JWT key cache with keys past 80% expiration
    let keys = EveJwtKeys::create_mock_keys();
    let timestamp = std::time::Instant::now() - std::time::Duration::from_secs(2881);
    esi_client.jwt_key_cache = Arc::new(RwLock::new(Some((keys, timestamp))));

    // Use get_jwt_keys as entry point since function being tested is private
    let _ = esi_client.oauth2().get_jwt_keys().await;

    // Wait for refresh notification or timeout if never completes
    let notify_future = esi_client.jwt_key_refresh_notifier.notified();
    let notify_timeout = Duration::from_millis(100);
    let notified = tokio::select! {
        _ = notify_future => {true}
        _ = tokio::time::sleep(notify_timeout) => {false}
    };

    // Assert notification has been received
    assert_eq!(notified, true);

    // Assert 1 request has been made to mock server
    mock.assert();

    // Assert refresh lock has been released
    let refresh_lock = &esi_client.jwt_key_refresh_lock;
    let lock_acquired = refresh_lock.compare_exchange(
        false,
        true,
        std::sync::atomic::Ordering::Acquire,
        std::sync::atomic::Ordering::Relaxed,
    );

    assert!(!lock_acquired.is_err());

    // Assert cache has been properly updated
    let cache = esi_client.jwt_key_cache.read().await;

    if let Some((_, timestamp)) = &*cache {
        // Ensure timestamp is not past default 2880 second nearing expiration mark
        let not_nearing_expired = timestamp.elapsed() < std::time::Duration::from_secs(2880);
        assert!(not_nearing_expired)
    } else {
        panic!("JWT keys cache is none, expected some keys present")
    }
}

/// Tests to ensure failure is handled properly by background refresh
///
/// When the JWT keys present in cache are approaching expiry,
/// being past 80% expired by default, the function should
/// trigger a background refresh. If the function fails to refresh
/// the keys, a failure attempt should be logged which helps
/// determine if the next attempt is within the backoff period.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a server response returning error 500 and expecting 1 request
/// - Populate cache with keys past 80% expired to trigger background refresh
/// - Call the function & wait for a refresh complete notification
///   or timeout if background refresh takes too long.
///
/// # Assertions
/// - Assert a notification has been received of completed refresh
/// - Assert 1 request has been made to mock server
/// - Assert refresh lock has been released
/// - Assert the failure attempt has been logged
/// - Assert the cache still contains keys nearing expiry
#[tokio::test]
async fn test_background_refresh_failure() {
    // Setup a basic EsiClient & mock HTTP server
    let (mut esi_client, mut mock_server) = setup().await;

    // Create mock response with error 500 and expecting 1 request
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 1);

    // Populate JWT key cache with keys past 80% expiration
    let keys = EveJwtKeys::create_mock_keys();
    let timestamp = std::time::Instant::now() - std::time::Duration::from_secs(2881);
    esi_client.jwt_key_cache = Arc::new(RwLock::new(Some((keys, timestamp))));

    // Use get_jwt_keys as entry point since function being tested is private
    let _ = esi_client.oauth2().get_jwt_keys().await;

    // Wait for refresh notification or timeout if never completes
    let notify_future = esi_client.jwt_key_refresh_notifier.notified();
    let notify_timeout = Duration::from_millis(100);
    let notified = tokio::select! {
        _ = notify_future => {true}
        _ = tokio::time::sleep(notify_timeout) => {false}
    };

    // Assert notification has been received
    assert_eq!(notified, true);

    // Assert 1 request has been made to mock server
    mock.assert();

    // Assert refresh lock has been released
    let refresh_lock = &esi_client.jwt_key_refresh_lock;
    let lock_acquired = refresh_lock.compare_exchange(
        false,
        true,
        std::sync::atomic::Ordering::Acquire,
        std::sync::atomic::Ordering::Relaxed,
    );

    assert!(!lock_acquired.is_err());

    // Assert last refresh failure has been logged
    let last_refresh_failure = esi_client.jwt_keys_last_refresh_failure.read().await;
    assert!(last_refresh_failure.is_some());

    // Assert cache still contains expired keys
    let cache = esi_client.jwt_key_cache.read().await;

    if let Some((_, timestamp)) = &*cache {
        // Ensure timestamp is past default 2880 second nearing expiration mark
        let nearing_expired = timestamp.elapsed() > std::time::Duration::from_secs(2880);
        assert!(nearing_expired)
    } else {
        panic!("JWT keys cache is none, expected some keys present")
    }
}

/// Tests the background refresh is keys are approaching expiry but backoff is present
///
/// When the JWT keys present in cache are approaching expiry,
/// being past 80% expired by default, the function would typically
/// trigger a background refresh but wouldn't due to a recent failure.
///
/// Since the recent failure is still within the default backoff period of
/// 100 miliseconds, no request would be made.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500 and expecting 0 requests
/// - Populate cache with keys past 80% expired to trigger background refresh
///
/// # Assertions
/// - Assert timed out waiting for background refresh as it should
///   not have been started
/// - Assert 0 requests have been made to mock server
#[tokio::test]
async fn test_background_refresh_backoff() {
    // Setup a basic EsiClient & mock HTTP server
    let (mut esi_client, mut mock_server) = setup().await;

    // Create mock response with error 500 and expecting 0 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

    // Populate JWT key cache with keys past 80% expiration
    let keys = EveJwtKeys::create_mock_keys();
    let expiration = std::time::Instant::now() - std::time::Duration::from_secs(2881);
    esi_client.jwt_key_cache = Arc::new(RwLock::new(Some((keys, expiration))));

    // Set last failure within backoff period of last 100 ms (failed 50 ms ago)
    let last_failure = std::time::Instant::now() - std::time::Duration::from_millis(50);
    esi_client.jwt_keys_last_refresh_failure = Arc::new(RwLock::new(Some(last_failure)));

    // Use get_jwt_keys as entry point since function being tested is private
    let _ = esi_client.oauth2().get_jwt_keys().await;

    // Wait for notification to timeout
    let notify_future = esi_client.jwt_key_refresh_notifier.notified();
    let notify_timeout = Duration::from_millis(100);
    let notified = tokio::select! {
        _ = notify_future => {true}
        _ = tokio::time::sleep(notify_timeout) => false
    };

    // Assert notified is false as no refresh task should've started
    assert_eq!(notified, false);

    // Assert 0 requests have been made to mock server
    mock.assert();
}

/// Tests the background refresh if refresh is already in progress by another thread
///
/// When the JWT keys present in cache are approaching expiry,
/// being past 80% expired by default, the function would typically
/// trigger a background refresh but wouldn't due to another thread
/// already handling the refresh.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500 and expecting 0 requests
/// - Populate cache with keys past 80% expired
/// - Acquire a refresh lock to simulate another thread handling the refresh
///
/// # Assertions
/// - Assert keys were returned from cache regardless of background
///   refresh success
/// - Assert timed out waiting for background refresh as it should
///   not have been started
/// - Assert 0 requests have been made to mock server
#[tokio::test]
async fn test_background_refresh_already_in_progress() {
    // Setup a basic EsiClient & mock HTTP server
    let (mut esi_client, mut mock_server) = setup().await;

    // Create mock response with error 500 and expecting 0 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

    // Populate JWT key cache with keys past 80% expiration
    let keys = EveJwtKeys::create_mock_keys();
    let expiration = std::time::Instant::now() - std::time::Duration::from_secs(2881);
    esi_client.jwt_key_cache = Arc::new(RwLock::new(Some((keys, expiration))));

    // Acquire a refresh lock
    let refresh_lock = &esi_client.jwt_key_refresh_lock;
    let lock_acquired = refresh_lock.compare_exchange(
        false,
        true,
        std::sync::atomic::Ordering::Acquire,
        std::sync::atomic::Ordering::Relaxed,
    );

    // Assert refresh lock is in place
    assert!(!lock_acquired.is_err());

    // Use get_jwt_keys as entry point since function being tested is private
    let _ = esi_client.oauth2().get_jwt_keys().await;

    // Wait for notification to timeout
    let notify_future = esi_client.jwt_key_refresh_notifier.notified();
    let notify_timeout = Duration::from_millis(100);
    let notified = tokio::select! {
        _ = notify_future => {true}
        _ = tokio::time::sleep(notify_timeout) => false
    };

    // Assert notified is false as no refresh task should've started
    assert_eq!(notified, false);

    // Assert 0 requests have been made to mock server
    mock.assert();
}
