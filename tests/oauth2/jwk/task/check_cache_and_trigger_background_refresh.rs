use std::sync::Arc;
use std::time::Duration;

use mockito::Server;
use tokio::sync::RwLock;

use eve_esi::model::oauth2::EveJwtKeys;
use eve_esi::EsiClient;

// These integration tests specifically check the logic of triggering the
// background refresh depending on the expiration status of the keys in the cache.
//
// Note: Need 100ms delay to accurately wait for background refresh to properly notify
// of completion. Maybe a refactor is necessary to avoid this wait for testing purposes?

// Test scenario for when cache has keys can be found at
// `jwk/get_jwt_keys: get_jwt_keys_valid_cache`

// Test scenario for when cache has no keys can be found at
// `jwk/get_jwt_keys: get_jwt_keys_expired_cache`

/// Tests the background refresh if keys are approaching expiry
///
/// When the JWT keys present in cache are approaching expiry,
/// being past 80% expired by default, the function should
/// trigger a background refresh.
///
/// # Test Setup
/// - Create a mock server which shouldn't get any requests
/// - Configures a server response returning expected keys
/// - Point the ESI client to the mock server URL for JWK endpoint
/// - Call the function & wait for a refresh complete notification
///   or timeout if background refresh takes too long.
///
/// # Assertions
/// - Assert a notification has been received of completed refresh
/// - Assert 1 request has been made to mock server
/// - Assert refresh lock has been released
/// - Assert cache has been properly updated with keys not nearing expiration
#[tokio::test]
async fn test_background_refresh_approaching_expiry() {
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
    let mut esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Populate JWT key cache with keys past 80% expiration
    let keys = EveJwtKeys::create_mock_keys();
    let timestamp = std::time::Instant::now() - std::time::Duration::from_secs(2881);
    esi_client.jwt_keys_cache = Arc::new(RwLock::new(Some((keys, timestamp))));

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
    let refresh_lock = &esi_client.jwt_key_refresh_in_progress;
    let lock_acquired = refresh_lock.compare_exchange(
        false,
        true,
        std::sync::atomic::Ordering::Acquire,
        std::sync::atomic::Ordering::Relaxed,
    );

    assert!(!lock_acquired.is_err());

    // Assert cache has been properly updated
    let cache = esi_client.jwt_keys_cache.read().await;

    if let Some((_, timestamp)) = &*cache {
        // Ensure timestamp is not past default 2880 second nearing expiration mark
        let not_nearing_expired = timestamp.elapsed() < std::time::Duration::from_secs(2880);
        assert!(not_nearing_expired)
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
/// - Create a mock server which shouldn't get any requests
/// - Configures a server response returning expected keys
/// - Point the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Assert timed out waiting for background refresh as it should
///   not have been started
/// - Assert 0 requests have been made to mock server
#[tokio::test]
async fn test_background_refresh_backoff() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response expecting 0 requests
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(0)
        .create();

    // Create ESI client with mock JWK endpoint
    let mut esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Populate JWT key cache with keys past 80% expiration
    let keys = EveJwtKeys::create_mock_keys();
    let expiration = std::time::Instant::now() - std::time::Duration::from_secs(2881);
    esi_client.jwt_keys_cache = Arc::new(RwLock::new(Some((keys, expiration))));

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
/// - Create a mock server which shouldn't get any requests
/// - Configures a server response returning expected keys
/// - Point the ESI client to the mock server URL for JWK endpoint
/// - Acquire a refresh lock to simulate another thread handling the refresh
///
/// # Assertions
/// - Assert timed out waiting for background refresh as it should
///   not have been started.
/// - Assert 0 requests have been made to mock server
#[tokio::test]
async fn test_background_refresh_already_in_progress() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response expecting 0 requests
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(0)
        .create();

    // Create ESI client with mock JWK endpoint
    let mut esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Populate JWT key cache with keys past 80% expiration
    let keys = EveJwtKeys::create_mock_keys();
    let expiration = std::time::Instant::now() - std::time::Duration::from_secs(2881);
    esi_client.jwt_keys_cache = Arc::new(RwLock::new(Some((keys, expiration))));

    // Acquire a refresh lock
    let refresh_lock = &esi_client.jwt_key_refresh_in_progress;
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
