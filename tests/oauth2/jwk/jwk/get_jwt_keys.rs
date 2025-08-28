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
        let mut cache = esi_client.jwt_key_cache.write().await;
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
        let mut cache = esi_client.jwt_key_cache.write().await;
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
        let cache = esi_client.jwt_key_cache.read().await;
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
        let cache = esi_client.jwt_key_cache.read().await;
        assert!(cache.is_some());
        let (cached_keys, _) = cache.as_ref().unwrap();
        // Verify we have the new keys in cache
        assert_eq!(cached_keys.keys.len(), new_expected_keys.keys.len());
    }
}
