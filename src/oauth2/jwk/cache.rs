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

use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
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
    ///
    /// # Related Methods
    /// High-Level
    /// - [`Self::get_jwt_keys`]: Public-facing method that uses this utility method
    /// - [`Self::fetch_and_update_cache`]: Updates the cache that this method reads from
    /// Task
    /// - [`Self::check_cache_and_trigger_background_refresh`]: Higher-level method that checks
    ///   expiration and may trigger background refresh
    /// Utility
    /// - [`Self::wait_for_ongoing_refresh`]: Used after detecting an ongoing refresh operation
    /// - [`Self::is_cache_expired`]: Can be used alongside this method to check validity
    pub(super) async fn cache_get_keys(&self) -> Option<(EveJwtKeys, std::time::Instant)> {
        #[cfg(not(tarpaulin_include))]
        trace!("Attempting to retrieve JWT keys from cache");

        // Retrieve the cache
        let cache = self.client.jwt_keys_cache.read().await;

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
    ///
    /// # Related Methods
    /// High-Level
    /// - [`Self::get_jwt_keys`]: Public-facing method that relies on this method for cache updates
    /// - [`Self::fetch_and_update_cache`]: Uses this method to update the cache with freshly fetched keys
    pub(super) async fn cache_update_keys(&self, keys: EveJwtKeys) {
        #[cfg(not(tarpaulin_include))]
        debug!(
            "Updating JWT keys cache with {} keys, ttl={}s",
            keys.keys.len(),
            self.client.jwt_keys_cache_ttl
        );

        let mut cache = self.client.jwt_keys_cache.write().await;
        *cache = Some((keys, std::time::Instant::now()));

        #[cfg(not(tarpaulin_include))]
        debug!("JWT keys cache successfully updated");
    }

    /// Attempts to atomically acquire the refresh lock for updating JWT keys
    ///
    /// This method uses an atomic compare-and-exchange operation to safely acquire
    /// the lock in a concurrent environment. It only succeeds if no other thread
    /// currently holds the lock (i.e., the atomic flag is false).
    ///
    /// # Implementation Details
    /// The lock acquisition uses an atomic compare-exchange operation with the following properties:
    /// - The operation succeeds only if the current value is `false` (no lock held)
    /// - Uses `Ordering::Acquire` for successful exchanges, ensuring all subsequent memory
    ///   operations by this thread cannot be reordered before this lock acquisition
    /// - Uses `Ordering::Relaxed` for failed exchanges since we don't care about
    ///   memory ordering guarantees when we fail to acquire the lock
    ///
    /// # Thread Safety
    /// This method is thread-safe and can be called concurrently from multiple threads.
    /// Only one thread will successfully acquire the lock. The atomic operation ensures
    /// there are no race conditions in determining which thread gets the lock.
    ///
    /// # Returns
    /// - [`true`] if the lock is acquired successfully,
    /// - [`false`] if the lock is already held by another thread
    ///
    /// # Related Methods
    ///
    /// ## High Level
    /// - [`Self::get_jwt_keys`]: Public-facing method that needs exclusive refresh access
    ///
    /// ## Task
    /// - [`Self::refresh_jwt_keys_with_retry`]: Uses this method to ensure exclusive refresh access
    ///
    /// ## Cache
    /// - [`Self::cache_lock_release_and_notify`]: Releases the lock acquired by this method
    pub(super) fn jwk_refresh_lock_try_acquire(&self) -> bool {
        let esi_client = self.client;
        let refresh_lock = &esi_client.jwt_key_refresh_in_progress;

        // Attempt to acquire a lock
        let lock_acquired = refresh_lock.compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::Acquire,
            std::sync::atomic::Ordering::Relaxed,
        );

        if !lock_acquired.is_err() {
            #[cfg(not(tarpaulin_include))]
            debug!("Successfully acquired JWT key refresh lock");

            // Lock successfully acquired
            true
        } else {
            #[cfg(not(tarpaulin_include))]
            trace!("Failed to acquire JWT key refresh lock (already held by another thread)");

            // Lock already in use
            false
        }
    }

    /// Releases the JWT key refresh lock and notifies any waiting threads
    ///
    /// This method is called after a JWT key refresh operation completes (either
    /// successfully or with an error). It performs two key actions:
    /// 1. Releases the atomic lock that prevents concurrent refresh operations
    /// 2. Notifies all threads waiting for the refresh operation to complete
    ///
    /// # Implementation Details
    /// - Uses atomic operations with [`std::sync::atomic::Ordering::Release`] to
    ///   ensure memory visibility
    /// - Calls [`tokio::sync::Notify::notify_waiters()`] on the notification channel
    ///   to wake up any threads that are blocked in [`Self::wait_for_ongoing_refresh()`]
    ///
    /// # Thread Safety
    /// This method is thread-safe and uses proper memory ordering to ensure that
    /// all memory operations performed during the refresh are visible to threads
    /// that subsequently acquire the lock or are notified.
    ///
    /// # Related Methods
    ///
    /// ## Cache
    /// - [`Self::cache_lock_try_acquire`]: The counterpart method used to acquire the lock
    /// - Any thread waiting on this notification will be unblocked within the timeout
    ///   period (default: 5 seconds) defined by [`DEFAULT_JWK_REFRESH_TIMEOUT`]
    ///
    /// ## Task
    /// - [`Self::refresh_jwt_keys_with_retry`]: Calls this method when refresh operations complete
    /// - [`Self::trigger_background_jwt_refresh`]: Calls this method in background tasks
    ///
    /// ## Utility
    /// - [`Self::wait_for_ongoing_refresh`]: Method used by threads waiting for notification
    pub(super) fn cache_lock_release_and_notify(&self) {
        let esi_client = self.client;
        let refresh_lock = &esi_client.jwt_key_refresh_in_progress;

        // Release the lock
        #[cfg(not(tarpaulin_include))]
        debug!("Releasing JWT key refresh lock");

        refresh_lock.store(false, std::sync::atomic::Ordering::Release);

        // Notify waiters
        #[cfg(not(tarpaulin_include))]
        trace!("Notifying waiters about JWT key refresh completion");

        self.client.jwt_key_refresh_notifier.notify_waiters();

        #[cfg(not(tarpaulin_include))]
        debug!("JWT key refresh lock released and waiters notified");
    }
}

#[cfg(test)]
mod cache_get_keys_tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::{model::oauth2::EveJwtKeys, EsiClient};

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
        esi_client.jwt_keys_cache = cache;

        // Test function
        let result = esi_client.oauth2().cache_get_keys().await;

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
        let result = esi_client.oauth2().cache_get_keys().await;

        // Assert None
        assert!(result.is_none())
    }
}

#[cfg(test)]
mod cache_update_keys_tests {
    use crate::{model::oauth2::EveJwtKeys, EsiClient};

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
        esi_client.oauth2().cache_update_keys(mock_keys).await;

        // Assert some
        let cache = esi_client.jwt_keys_cache.read().await;
        let result = &*cache;

        assert!(result.is_some())
    }
}

#[cfg(test)]
mod cache_lock_try_acquire_tests {
    use crate::EsiClient;

    /// Checks that lock is properly acquired when not already in use
    ///
    /// Attempts to acquire a lock to refresh JWT keys on a new
    /// EsiClient which should return as successful (true) indicating
    /// that no other threads are currently attempting a key refresh.
    ///
    /// # Test Setup
    /// - Setup a basic EsiClient
    /// - Attempt to acquire a lock for JWT key refresh
    ///
    /// # Assertions
    /// - Assert that result is true when acquiring lock
    #[test]
    fn test_jwk_refresh_lock_try_acquire_success() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Attempt to acquire lock
        let lock_acquired = esi_client.oauth2().jwk_refresh_lock_try_acquire();

        // Assert
        assert_eq!(lock_acquired, true)
    }

    /// Checks that lock is not acquired when already in use
    ///
    /// Acquires a lock initially and then attempts to acquire a lock
    /// again despite it already being in use which should return as
    /// unsuccessful (false).
    ///
    /// # Test Setup
    /// - Setup basic EsiClient
    /// - Acquire a lock initially
    /// - Attempt to acquire lock again
    ///
    /// # Assertions
    /// - Asserts that result is false when attempting to acquire lock
    ///   a second time indicating it is already in use.
    #[test]
    fn test_jwk_refresh_lock_try_acquire_unsuccessful() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Acquire a lock initially
        let lock = esi_client.oauth2().jwk_refresh_lock_try_acquire();

        if !lock {
            panic!("Failed to acquire initial lock")
        }

        // Acquire lock a second time
        // Should return false indicating lock is already in use
        let lock_acquired = esi_client.oauth2().jwk_refresh_lock_try_acquire();

        // Assert
        assert_eq!(lock_acquired, false)
    }
}

#[cfg(test)]
mod cache_lock_release_and_notify_tests {
    use std::time::Duration;

    use crate::EsiClient;

    /// Verifies that lock is successfully released & waiters are notified
    ///
    /// Acquires a lock and sets up a notification listener which listens
    /// for the notification of when the lock is released. If the notification
    /// is never received than the listener will timeout. Checks to ensure that
    /// notification was properly received as well as the lock was released
    /// without issues.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Acquire a JWT key refresh lock
    /// - Setup a notification listener
    ///
    /// # Assertions
    /// - Assert that lock has been properly acquired
    /// - Assert that lock release notification was received
    /// - Assert that lock has been properly released
    #[tokio::test]
    async fn test_cache_lock_release_and_notify_success() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Acquire a lock
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

        // Create the notification future BEFORE triggering release
        let notification = esi_client.jwt_key_refresh_notifier.notified();
        let timeout = tokio::time::sleep(Duration::from_millis(50));

        // Release and notify
        esi_client.oauth2().cache_lock_release_and_notify();

        let notified = tokio::select! {
            _ = notification => {
                // Notification received successfully
                true
            }
            _ = timeout => {
                // Timed out waiting for notification
                false
            }
        };

        // Assert that notification was received
        assert_eq!(notified, true);

        // Assert that lock has been released and can be acquired again
        let lock = !esi_client
            .jwt_key_refresh_in_progress
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err();

        assert_eq!(lock, true)
    }
}
