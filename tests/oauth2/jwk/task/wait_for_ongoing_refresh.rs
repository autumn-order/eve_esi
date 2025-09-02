use eve_esi::error::{EsiError, OAuthError};
use eve_esi::model::oauth2::EveJwtKeys;

use crate::oauth2::jwk::util::{get_jwk_internal_server_error_response, setup};

/// Validates retrieving keys from cache after waiting for refresh.
///
/// Simulates waiting for another thread to finish refreshing JWT keys
/// by acquiring a refresh lock and using a coroutine to simulate the
/// refresh. Validates that the function properly returns expected
/// keys after refresh finishes.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500 and expecting 0 requests
/// - Acquire a lock on refreshing JWT keys
/// - Spawn a coroutine to simulate another thread refreshing the keys
///
/// # Assertions
/// - Assert that refresh lock is in place
/// - Assert no requests have been made to mock JWK endpoint
/// - Assert that expected keys have been returned by the function
#[tokio::test]
async fn test_wait_for_refresh_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;
    let jwt_key_cache = &esi_client.jwt_key_cache;

    // Create mock response with error 500 and expecting 0 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

    // Acquire a refresh lock
    let lock = !jwt_key_cache
        .refresh_lock
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::Acquire,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_err();

    // Assert that lock is in place
    assert_eq!(lock, true);

    // Create a channel to listen for when the coroutine starts
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn a coroutine to perform the background refresh
    let keys = EveJwtKeys::create_mock_keys();

    let keys_clone = keys.clone();
    let cache_clone = esi_client.jwt_key_cache.clone();

    tokio::spawn(async move {
        // Signal that refresh is about to start
        let _ = tx.send(());

        // Simulate a network request delay
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Update keys
        let mut cache = cache_clone.cache.write().await;
        *cache = Some((keys_clone, std::time::Instant::now()));

        // Release lock & notify waiters
        cache_clone
            .refresh_lock
            .store(false, std::sync::atomic::Ordering::Release);

        cache_clone.refresh_notifier.notify_waiters();
    });

    // Wait for coroutine to begin refresh
    rx.await.expect("Failed to receive ready signal");

    // Use get_jwt_keys as entry point since function being tested is private
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert mock server received 0 requests
    mock.assert();

    // Check that we got the expected keys
    assert!(result.is_ok());
    let jwt_keys = result.unwrap();
    assert_eq!(jwt_keys.keys.len(), keys.keys.len());
}

/// Validates error handling when the ongoing refresh fails
///
/// Simulates waiting for another thread to finish refreshing JWT keys
/// by acquiring a refresh lock and using a coroutine to simulate the
/// refresh. Validates that the error is properly handled when the
/// JWT key cache is not updated due to a refresh failure.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500 and expecting 0 requests
/// - Acquire a lock on refreshing JWT keys
/// - Spawn a coroutine to simulate another thread failing to
///   refresh the keys
///
/// # Assertions
/// - Assert that refresh lock is in place
/// - Assert no requests have been made to mock JWK endpoint
/// - Assert that an OAuthError::JwtKeyCacheError has been returned
#[tokio::test]
async fn test_wait_for_refresh_failure() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;
    let jwt_key_cache = &esi_client.jwt_key_cache;

    // Create mock response with error 500 and expecting 0 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

    // Acquire a refresh lock
    let lock = !jwt_key_cache
        .refresh_lock
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::Acquire,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_err();

    // Assert that lock is in place
    assert_eq!(lock, true);

    // Create a channel to listen for when the coroutine starts
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn a coroutine to perform the background refresh
    let cache_clone = jwt_key_cache.clone();

    tokio::spawn(async move {
        // Signal that refresh is about to start
        let _ = tx.send(());

        // Simulate a network request delay
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Don't update the cache with keys to represent a failure

        // Release lock & notify waiters regardless of success
        cache_clone
            .refresh_lock
            .store(false, std::sync::atomic::Ordering::Release);

        cache_clone.refresh_notifier.notify_waiters();
    });

    // Wait for coroutine to begin refresh
    rx.await.expect("Failed to receive ready signal");

    // Use get_jwt_keys as entry point since function being tested is private
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert mock server received 0 requests
    mock.assert();

    // Assert function returned expected error
    assert!(result.is_err());
    match result {
        Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(_))) => {}
        _ => panic!("Expected OAuthError::JwtKeyCacheError, got different error type"),
    }
}

/// Validates error handling when a timeout occurs waiting for refresh
///
/// Simulates waiting for another thread to finish refreshing JWT keys
/// before returning an error when the function times out waiting for
/// the refresh that never finishes.
///
/// # Test Setup
/// - Create a basic EsiClient & mock HTTP server
/// - Configures a mock response returning an error 500 and expecting 0 requests
/// - Acquire a lock on refreshing JWT keys
/// - Cause a timeout by never notifying of a completed refresh
///
/// # Assertions
/// - Assert that refresh lock is in place
/// - Assert no requests have been made to mock JWK endpoint
/// - Assert that an OAuthError::JwtKeyCacheError has been returned
#[tokio::test]
async fn test_wait_for_refresh_timeout() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;
    let jwt_key_cache = &esi_client.jwt_key_cache;

    // Create mock response with error 500 and expecting 0 requests
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

    // Acquire a refresh lock
    let lock = !jwt_key_cache
        .refresh_lock
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::Acquire,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_err();

    // Assert that lock is in place
    assert_eq!(lock, true);

    // Don't attempt any cache updates and don't release the lock which
    // should cause a timeout error.

    // Use get_jwt_keys as entry point since function being tested is private
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert mock server received 0 requests
    mock.assert();

    // Assert function returned expected error
    assert!(result.is_err());
    match result {
        Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(_))) => {}
        _ => panic!("Expected OAuthError::JwtKeyCacheError, got different error type"),
    }
}
