use eve_esi::error::{EsiError, OAuthError};
use eve_esi::model::oauth2::EveJwtKeys;
use eve_esi::EsiClient;
use mockito::Server;

/// Validates retrieving keys from cache after waiting for refresh.
///
/// Simulates waiting for another thread to finish refreshing JWT keys
/// by acquiring a refresh lock and using a coroutine to simulate the
/// refresh. Validates that the function properly returns expected
/// keys after refresh finishes.
///
/// # Test Setup
/// - Create a mock server which shouldn't get any requests
/// - Configures a server response expecting 0 requests
/// - Point the ESI client to the mock server URL for JWK endpoint
/// - Acquire a lock on refreshing JWT keys
/// - Spawn a coroutine to simulate another thread refreshing the keys
///
/// # Assertions
/// - Assert that refresh lock is in place
/// - Assert no requests have been made to mock JWK endpoint
/// - Assert that expected keys have been returned by the function
#[tokio::test]
async fn test_wait_for_refresh_success() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response returning status 500 and
    // expecting 0 requests
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(0)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Acquire a refresh lock
    let lock = !esi_client
        .jwt_key_refresh_in_progress
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
    let jwt_keys_cache = esi_client.jwt_keys_cache.clone();
    let jwt_key_refresh_lock = esi_client.jwt_key_refresh_in_progress.clone();
    let jwt_key_refresh_notifier = esi_client.jwt_key_refresh_notifier.clone();
    tokio::spawn(async move {
        // Signal that refresh is about to start
        let _ = tx.send(());

        // Simulate a network request delay
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Update keys
        let mut cache = jwt_keys_cache.write().await;
        *cache = Some((keys_clone, std::time::Instant::now()));

        // Release lock & notify waiters
        jwt_key_refresh_lock.store(false, std::sync::atomic::Ordering::Release);

        jwt_key_refresh_notifier.notify_waiters();
    });

    // Wait for coroutine to begin refresh
    rx.await.expect("Failed to receive ready signal");

    // Call the get_jwt_keys method
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
/// - Create a mock server which shouldn't get any requests
/// - Configures a server response expecting 0 requests
/// - Point the ESI client to the mock server URL for JWK endpoint
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
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response returning status 500 and
    // expecting 0 requests
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(0)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Acquire a refresh lock
    let lock = !esi_client
        .jwt_key_refresh_in_progress
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
    let jwt_key_refresh_lock = esi_client.jwt_key_refresh_in_progress.clone();
    let jwt_key_refresh_notifier = esi_client.jwt_key_refresh_notifier.clone();
    tokio::spawn(async move {
        // Signal that refresh is about to start
        let _ = tx.send(());

        // Simulate a network request delay
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Don't update the cache with keys to represent a failure

        // Release lock & notify waiters regardless of success
        jwt_key_refresh_lock.store(false, std::sync::atomic::Ordering::Release);

        jwt_key_refresh_notifier.notify_waiters();
    });

    // Wait for coroutine to begin refresh
    rx.await.expect("Failed to receive ready signal");

    // Call the get_jwt_keys method
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
/// - Create a mock server which shouldn't get any requests
/// - Configures a server response expecting 0 requests
/// - Point the ESI client to the mock server URL for JWK endpoint
/// - Acquire a lock on refreshing JWT keys
/// - Cause a timeout by never notifying of a completed refresh
///
/// # Assertions
/// - Assert that refresh lock is in place
/// - Assert no requests have been made to mock JWK endpoint
/// - Assert that an OAuthError::JwtKeyCacheError has been returned
#[tokio::test]
async fn test_wait_for_refresh_timeout() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response returning status 500 and
    // expecting 0 requests
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal Server Error"}"#)
        .expect(0)
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Acquire a refresh lock
    let lock = !esi_client
        .jwt_key_refresh_in_progress
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

    // Use get_jwt_keys as entry point since wait_for_ongoing_refresh
    // is private
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
