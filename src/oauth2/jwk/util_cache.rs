//! JWT Key Cache Management
//!
//! This module implements the caching mechanisms for JWT keys, including:
//! - Direct cache access functions
//! - Cache update operations
//! - Lock management for thread-safe cache access
//! - Cache invalidation strategies
//!
//! The caching system uses RwLocks for efficient concurrent reads with exclusive writes
//! and atomic operations for coordinating refresh operations across threads.
//!
//! See the [module-level documentation](super) for a more detailed overview and usage.

use log::{debug, trace};

use crate::client::JwtKeyCache;
use crate::model::oauth2::EveJwtKeys;

/// Retrieves JWT keys directly from cache without validation or refresh attempts
///
/// This is a low-level utility method that provides direct access to the JWT keys
/// stored in the cache. Unlike higher-level methods such as
/// [`Self::check_cache_and_trigger_background_refresh`], this method:
///
/// - Does not check if the cached keys are expired
/// - Does not trigger background refresh tasks
/// - Does not attempt to fetch new keys if the cache is empty
/// - Acquires only a read lock (safe for concurrent access)
///
/// # Use Cases
///
/// - Use when you need quick access to keys and expiration doesn't matter
/// - Use after a refresh operation when you know the cache should be populated
/// - Use when you've already checked validity elsewhere and just need the keys
/// - Use when implementing custom caching logic that needs the raw keys
///
/// # Thread Safety
///
/// This method acquires a read lock on the cache, allowing multiple concurrent
/// readers without blocking each other. The lock is automatically released when
/// the method returns.
///
/// # Returns
/// - Some([`EveJwtKeys`]) if keys are present in the cache (valid or not)
/// - [`None`] if the cache is empty (no keys have been fetched yet). This typically
///   triggers a fetch operation with retry logic when called from higher-level methods.
pub(super) async fn cache_get_keys(
    jwt_key_cache: &JwtKeyCache,
) -> Option<(EveJwtKeys, std::time::Instant)> {
    #[cfg(not(tarpaulin_include))]
    trace!("Attempting to retrieve JWT keys from cache");

    // Retrieve the cache
    let cache = jwt_key_cache.read().await;

    // Check if the cache has keys stored
    if let Some((keys, timestamp)) = &*cache {
        let elapsed = timestamp.elapsed().as_secs();

        #[cfg(not(tarpaulin_include))]
        trace!(
            "Found JWT keys in cache: key_count={}, elapsed={}s",
            keys.keys.len(),
            elapsed
        );

        // Return the keys found in cache
        return Some((keys.clone(), timestamp.clone()));
    }

    #[cfg(not(tarpaulin_include))]
    debug!("JWT keys cache is empty, keys need to be fetched");

    // Return None since no data was found in the cache
    None
}

/// Updates the JWT keys cache with new keys and the current timestamp
///
/// Stores the provided JWT keys in the cache along with the current timestamp,
/// which will be used to determine when the keys should be refreshed next.
///
/// # Implementation Details
/// - Acquires a write lock on the JWT keys cache
/// - Stores the keys along with the current timestamp as an `Instant`
/// - The timestamp is used to calculate expiration based on the configured
///   TTL (default: 3600 seconds/1 hour)
///
/// # Thread Safety
/// This method acquires a write lock on the cache, ensuring that no other
/// thread can read or write to the cache while the update is in progress.
/// The lock is automatically released when the method returns.
///
/// # Parameters
/// - `keys`: The EVE JWT keys to store in the cache
pub(super) async fn cache_update_keys(jwt_key_cache: &JwtKeyCache, keys: EveJwtKeys) {
    #[cfg(not(tarpaulin_include))]
    debug!("Updating JWT keys cache with {} keys", keys.keys.len());

    let mut cache = jwt_key_cache.write().await;
    *cache = Some((keys, std::time::Instant::now()));

    #[cfg(not(tarpaulin_include))]
    debug!("JWT keys cache successfully updated");
}

#[cfg(test)]
mod cache_get_keys_tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::{model::oauth2::EveJwtKeys, EsiClient};

    use super::cache_get_keys;

    /// Validates function returns Some keys when cache has keys
    ///
    /// Checks that when the cache has keys set, the cache_get_keys
    /// function returns them properly without issues.
    ///
    /// # Test Setup
    /// - Setup basic EsiClient
    /// - Set EsiClient JWT key cache with mock keys
    ///
    /// # Assertions
    /// - Verify function returns Some(EveJwtKeys)
    #[tokio::test]
    async fn test_cache_get_keys_some() {
        // Setup basic EsiClient
        let mut esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Set JWT key cache
        let keys = (EveJwtKeys::create_mock_keys(), std::time::Instant::now());
        let cache = Arc::new(RwLock::new(Some(keys)));
        esi_client.jwt_key_cache = cache;

        // Test function
        let result = cache_get_keys(&esi_client.jwt_key_cache).await;

        // Assert Some
        assert!(result.is_some())
    }

    /// Validates function returns none when cache is empty
    ///
    /// Checks that when the cache is empty, the cache_get_jeys
    /// function returns None as expected.
    ///
    /// # Test Setup
    /// - Setup basic EsiClient
    /// - Do not set the JWT key cache
    ///
    /// # Assertions
    /// - Verify function returns None
    #[tokio::test]
    async fn test_cache_get_keys_none() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Do not set JWT key cache which is None by default

        // Test function
        let result = cache_get_keys(&esi_client.jwt_key_cache).await;

        // Assert None
        assert!(result.is_none())
    }
}

#[cfg(test)]
mod cache_update_keys_tests {
    use crate::{model::oauth2::EveJwtKeys, EsiClient};

    use super::cache_update_keys;

    /// Validates that cache properly updates with new keys
    ///
    /// Checks that writing new keys to the JWT key cache on
    /// EsiClient is successful.
    ///
    /// # Test Setup
    /// - Setup basic EsiClient
    /// - Create mock JWT keys
    ///
    /// # Assertions
    /// - Assert that the EsiClient jwt_keys_cache now is Some()
    #[tokio::test]
    async fn test_cache_update_keys() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Create mock keys
        let mock_keys = EveJwtKeys::create_mock_keys();

        // Test function
        cache_update_keys(&esi_client.jwt_key_cache, mock_keys).await;

        // Assert some
        let cache = esi_client.jwt_key_cache.read().await;
        let result = &*cache;

        assert!(result.is_some())
    }
}
