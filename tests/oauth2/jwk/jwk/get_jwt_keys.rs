use std::time::Instant;

use eve_esi::model::oauth2::EveJwtKeys;
use eve_esi::EsiClient;
use mockito::Server;

use super::super::mock::create_mock_jwt_keys_alternative;

/// Tests that get_jwt_keys returns cached keys when they are not expired.
///
/// # Test Setup
/// - Creates a mock server to simulate the EVE JWK endpoint
/// - Configures a mock response (which should NOT be called)
/// - Points the ESI client to the mock server URL for JWK endpoint
/// - Pre-populates the cache with fresh keys
///
/// # Assertions
/// - Verifies that no fetch request is made
/// - Verifies that the returned JWT keys match the cached keys
#[tokio::test]
async fn get_jwt_keys_valid_cache() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create cached JWT keys
    let cached_keys = EveJwtKeys::create_mock_keys();

    // Create a mock that should NOT be called (will fail test if called)
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .expect(0) // Expect no calls
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Pre-populate the cache
    {
        let mut cache = esi_client.jwt_keys_cache.write().await;
        *cache = Some((cached_keys.clone(), Instant::now()));
    }

    // Call the method under test
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert
    assert!(result.is_ok());
    let jwt_keys = result.unwrap();
    assert_eq!(
        jwt_keys.skip_unresolved_json_web_keys,
        cached_keys.skip_unresolved_json_web_keys
    );
    assert_eq!(jwt_keys.keys.len(), cached_keys.keys.len());

    // Verify that mock was not called (implicit in expect(0))
    mock.assert();
}

/// Tests that get_jwt_keys fetches fresh keys when the cache is expired.
///
/// # Test Setup
/// - Creates a mock server to simulate the EVE JWK endpoint
/// - Configures a mock response with expected JWT keys
/// - Points the ESI client to the mock server URL for JWK endpoint
/// - Pre-populates the cache with expired keys
///
/// # Assertions
/// - Verifies that the fetch request is made
/// - Verifies that the returned JWT keys match the new keys
/// - Verifies that the cache is updated after the call
#[tokio::test]
async fn get_jwt_keys_expired_cache() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create old cached keys and new expected keys
    let old_cached_keys = EveJwtKeys::create_mock_keys();
    let new_expected_keys = create_mock_jwt_keys_alternative();

    // Create mock response with new keys
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&new_expected_keys).unwrap())
        .create();

    // Create ESI client with mock JWK endpoint and custom cache TTL (0 seconds = always expired)
    let mut esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Set a very short TTL to force cache expiration
    esi_client.jwt_keys_cache_ttl = 0;

    // Pre-populate the cache with old data
    {
        let mut cache = esi_client.jwt_keys_cache.write().await;
        *cache = Some((old_cached_keys.clone(), Instant::now()));
    }

    // Call the method under test
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert
    mock.assert(); // Verify the refresh request was made
    assert!(result.is_ok());

    // Check that we got the new keys
    let jwt_keys = result.unwrap();
    assert_eq!(jwt_keys.keys.len(), new_expected_keys.keys.len());

    // Cache should be updated with new keys
    {
        let cache = esi_client.jwt_keys_cache.read().await;
        assert!(cache.is_some());
        let (cached_keys, _) = cache.as_ref().unwrap();
        // Verify we have the new keys in cache
        assert_eq!(cached_keys.keys.len(), new_expected_keys.keys.len());
    }
}

/// Tests that get_jwt_keys fetches fresh keys when the cache is empty.
///
/// # Test Setup
/// - Creates a mock server to simulate the EVE JWK endpoint
/// - Configures a mock response with expected JWT keys
/// - Points the ESI client to the mock server URL for JWK endpoint
///
/// # Assertions
/// - Verifies that the fetch request is made
/// - Verifies that the returned JWT keys match the new keys
/// - Verifies that the cache is updated after the call
#[tokio::test]
async fn get_jwt_keys_empty_cache() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create new expected keys
    let new_expected_keys = create_mock_jwt_keys_alternative();

    // Create mock response with keys
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&new_expected_keys).unwrap())
        .create();

    // Create ESI client with mock JWK endpoint
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .jwk_url(&format!("{}/oauth/jwks", mock_server_url))
        .build()
        .expect("Failed to build EsiClient");

    // Call the method under test
    let result = esi_client.oauth2().get_jwt_keys().await;

    // Assert
    mock.assert(); // Verify the refresh request was made
    assert!(result.is_ok());

    // Check that we got the new keys
    let jwt_keys = result.unwrap();
    assert_eq!(jwt_keys.keys.len(), new_expected_keys.keys.len());

    // Cache should be updated with new keys
    {
        let cache = esi_client.jwt_keys_cache.read().await;
        assert!(cache.is_some());
        let (cached_keys, _) = cache.as_ref().unwrap();
        // Verify we have the new keys in cache
        assert_eq!(cached_keys.keys.len(), new_expected_keys.keys.len());
    }
}

/// Tests that get_jwt_keys properly waits for already ongoing key refresh
///
/// Acquires a refresh lock and uses a coroutine to simulate performing a
/// JWT key refresh on another thread. Expect get_jwt_keys to wait for the
/// notification that the refresh is complete before returning the updated
/// JWT keys.
///
/// # Test Setup
/// - Creates a mock server to simulate the EVE JWK endpoint
/// - Configures a mock response with expected JWT keys which should not get
///   any requests from the get_jwt_keys method.
/// - Points the ESI client to the mock server URL for JWK endpoint
/// - Spawn a new thread which simulates a key refresh with a 50ms delay
/// - Wait for the thread to start before executing code
///
/// # Assertions
/// - Verifies that the fetch request is made
/// - Verifies that the returned JWT keys match the new keys
/// - Verifies that the cache is updated after the call
#[tokio::test]
async fn test_get_jwt_keys_ongoing_refresh() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response with keys which should not get any requests
    // from the get_jwt_keys method call
    let keys = EveJwtKeys::create_mock_keys();
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&keys).unwrap())
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
    let jwt_keys_cache = esi_client.jwt_keys_cache.clone();
    let jwt_key_refresh_lock = esi_client.jwt_key_refresh_in_progress.clone();
    let jwt_key_refresh_notifier = esi_client.jwt_key_refresh_notifier.clone();
    let keys_clone = keys.clone();
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

    // Assert
    mock.assert(); // Verify that no requests were made
    assert!(result.is_ok());

    // Check that we got the new keys
    let jwt_keys = result.unwrap();
    assert_eq!(jwt_keys.keys.len(), keys.keys.len());

    // Cache should be updated with new keys
    {
        let cache = esi_client.jwt_keys_cache.read().await;
        assert!(cache.is_some());
        let (cached_keys, _) = cache.as_ref().unwrap();
        // Verify we have the new keys in cache
        assert_eq!(cached_keys.keys.len(), keys.keys.len());
    }
}
