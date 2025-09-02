//! Provides the JwtKeyCache struct for caching JWT keys
//!
//! This module implements the caching mechanisms for JWT keys, including:
//! - Direct cache access functions
//! - Cache update operations
//! - Lock management for thread-safe cache access
//! - Cache invalidation strategies
//! - JWT key refresh locks
//! - Notification of completed refreshes
//!
//! The caching system uses RwLocks for efficient concurrent reads with exclusive writes
//! and atomic operations for coordinating refresh operations across threads.
//!
//! For details, see the [`JwtKeyCache`] struct.
//! For a higher level overview of the usage of JWT keys, see [module-level documentation](super)

use std::sync::atomic::AtomicBool;
use std::time::Instant;

use log::{debug, trace};
use tokio::sync::{Notify, RwLock};

use crate::model::oauth2::EveJwtKeys;

/// OAuth2 JWT key cache
///
/// A cache providing a tuple of [`EveJwtKeys`] and an [`Instant`] timestamp of when the keys
/// were last updated.
///
/// Used by methods [`get_jwt_keys`](crate::oauth2::OAuth2Api::get_jwt_keys) &
/// [`fetch_and_update_cache`](crate::oauth2::OAuth2Api::fetch_and_update_cache) to cache & refresh
/// JWT keys used to validate tokens retrieved from EVE Online's OAuth2 API.
///
/// Provides fields used to coordinate concurrency across multiple theads such as simulatenous reads,
/// acquiring a lock to prevent duplicate refresh attempts, and a notifier for when a refresh completes.
///
/// # Concurrency
/// - [`RwLock`]: To allow for simultaneous reads of the cache and the last refresh failure timestamp
/// - [`AtomicBool`]: To manage a high volume of simultaneous attempts to acquire a refresh lock
/// - [`Notify`]: To provide notifications of when the cache has been updated
///
/// # Fields
/// - `jwt_key_cache` (RwLock<Option<([`EveJwtKeys`], [`Instant`])>>): RwLock with a tuple containing JWT keys and timestamp of when keys were updated
/// - `jwt_key_refresh_lock` ([`AtomicBool`]): AtomicBool indicating whether a JWT key refresh is currently in progress
/// - `jwt_key_refresh_notifier` ([`Notify`]): Notifier for when a JWT key refresh is completed
/// - `jwt_key_last_refresh_failure` (RwLock<Option<[`Instant`]>): RwLock with a timestamp of last failed set of JWT key refresh attemmpts
pub struct JwtKeyCache {
    /// RwLock with a tuple containing JWT keys and timestamp of when keys were updated
    pub cache: RwLock<Option<(EveJwtKeys, Instant)>>,
    /// AtomicBool indicating whether a JWT key refresh is currently in progress
    pub refresh_lock: AtomicBool,
    /// Notifier for when a JWT key refresh is completed
    pub refresh_notifier: Notify,
    /// RwLock with a timestamp of last failed set of JWT key refresh attemmpts
    pub last_refresh_failure: RwLock<Option<Instant>>,
}

impl JwtKeyCache {
    /// Creates a new instance of [`JwtKeyCache`]
    ///
    /// The cache will start empty and will need to be updated using one of the update
    /// methods such as [`get_jwt_keys`](crate::oauth2::OAuth2Api::get_jwt_keys)
    /// or [`fetch_and_update_cache`](crate::oauth2::OAuth2Api::fetch_and_update_cache).
    ///
    /// # Returns
    /// - [`JwtKeyCache`]: Default cache instance that contains no keys initially
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(None),
            refresh_lock: AtomicBool::new(false),
            refresh_notifier: Notify::new(),
            last_refresh_failure: RwLock::new(None),
        }
    }

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
    pub(super) async fn get_keys(self: &Self) -> Option<(EveJwtKeys, std::time::Instant)> {
        #[cfg(not(tarpaulin_include))]
        trace!("Attempting to retrieve JWT keys from cache");

        // Retrieve the cache
        let cache = self.cache.read().await;

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
    pub(super) async fn update_keys(self: &Self, keys: EveJwtKeys) {
        #[cfg(not(tarpaulin_include))]
        debug!("Updating JWT keys cache with {} keys", keys.keys.len());

        let mut cache = self.cache.write().await;
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
    pub(super) fn refresh_lock_try_acquire(self: &Self) -> bool {
        // Attempt to acquire a lock
        let lock_acquired = self.refresh_lock.compare_exchange(
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
    pub(super) fn refresh_lock_release_and_notify(self: &Self) {
        // Release the lock
        #[cfg(not(tarpaulin_include))]
        debug!("Releasing JWT key refresh lock");

        self.refresh_lock
            .store(false, std::sync::atomic::Ordering::Release);

        // Notify waiters
        #[cfg(not(tarpaulin_include))]
        trace!("Notifying waiters about JWT key refresh completion");

        self.refresh_notifier.notify_waiters();

        #[cfg(not(tarpaulin_include))]
        debug!("JWT key refresh lock released and waiters notified");
    }

    /// Sets the last JWT key cache refresh failure time
    ///
    /// Updates the last failure time for a JWT key cache refresh which is used to determine
    /// if a refresh failure recently occurred within the 60 second cooldown period.
    ///
    /// The [`check_refresh_cooldown`](super::util::check_refresh_cooldown) utility function
    /// can be used check the remaining time on the cooldown period if applicable.
    ///
    /// # Implementation Details
    /// - Acquires a write lock on the [`JwtKeyCache::last_refresh_failure`] field
    ///   and updates with the provided timestamp or None.
    ///
    /// # Arguments
    /// - `failure_timestamp` (Option<[`Instant`]>): Option representing the last refresh failure time
    pub(super) async fn set_refresh_failure(self: &Self, failure_timstamp: Option<Instant>) {
        let mut failure_time = self.last_refresh_failure.write().await;
        *failure_time = failure_timstamp;
    }
}

#[cfg(test)]
mod cache_get_keys_tests {
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
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        let jwt_key_cache = esi_client.jwt_key_cache;

        // Set JWT key cache
        {
            let keys = (EveJwtKeys::create_mock_keys(), std::time::Instant::now());

            let mut cache = jwt_key_cache.cache.write().await;
            *cache = Some(keys);
        }

        // Test function
        let result = jwt_key_cache.get_keys().await;

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

        let jwt_key_cache = esi_client.jwt_key_cache;

        // Do not set JWT key cache which is None by default

        // Test function
        let result = jwt_key_cache.get_keys().await;

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

        let jwt_key_cache = esi_client.jwt_key_cache;

        // Create mock keys
        let mock_keys = EveJwtKeys::create_mock_keys();

        // Test function
        jwt_key_cache.update_keys(mock_keys).await;

        // Assert some
        let cache = jwt_key_cache.cache.read().await;
        let result = &*cache;

        assert!(result.is_some())
    }
}

#[cfg(test)]
mod jwk_refresh_lock_try_acquire_tests {
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
        let jwt_key_cache = &esi_client.jwt_key_cache;

        let lock_acquired = jwt_key_cache.refresh_lock_try_acquire();

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
        let jwt_key_cache = &esi_client.jwt_key_cache;

        let lock = jwt_key_cache.refresh_lock_try_acquire();

        if !lock {
            panic!("Failed to acquire initial lock")
        }

        // Acquire lock a second time
        // Should return false indicating lock is already in use
        let jwt_key_cache = &esi_client.jwt_key_cache;

        let lock_acquired = jwt_key_cache.refresh_lock_try_acquire();

        // Assert
        assert_eq!(lock_acquired, false)
    }
}

#[cfg(test)]
mod jwk_lock_release_and_notify_tests {
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
    async fn test_jwk_refresh_lock_release_and_notify_success() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Acquire a lock
        let jwt_key_cache = &esi_client.jwt_key_cache;

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

        // Create the notification future BEFORE triggering release
        let notification = jwt_key_cache.refresh_notifier.notified();
        let timeout = tokio::time::sleep(Duration::from_millis(50));

        // Release and notify
        jwt_key_cache.refresh_lock_release_and_notify();

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
        let lock = !jwt_key_cache
            .refresh_lock
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

#[cfg(test)]
mod set_refresh_failure_tests {
    use crate::EsiClient;

    /// Set the last refresh failure timestamp
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    ///
    /// # Assertions
    /// - Assert last refresh failure is Some
    /// - Assert failure time matches timestamp set
    #[tokio::test]
    async fn test_set_refresh_failure_some() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        let jwt_key_cache = &esi_client.jwt_key_cache;

        // Call function
        let timestamp = std::time::Instant::now();
        jwt_key_cache.set_refresh_failure(Some(timestamp)).await;

        // Assert last refresh failure is Some
        let result = jwt_key_cache.last_refresh_failure.read().await;
        assert!(result.is_some());

        // Assert failure time matches timestamp set
        let failure_time = result.unwrap();
        assert_eq!(failure_time, timestamp)
    }

    /// Set the last refresh failure timestamp to none
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Set a refresh failure timestamp
    ///
    /// # Assertions
    /// - Assert last refresh failure is None
    #[tokio::test]
    async fn test_set_refresh_failure_none() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        let jwt_key_cache = &esi_client.jwt_key_cache;

        // Set a refresh failure_timestamp
        {
            let mut failure_time = jwt_key_cache.last_refresh_failure.write().await;
            *failure_time = Some(std::time::Instant::now())
        }

        // Call function
        jwt_key_cache.set_refresh_failure(None).await;

        // Assert last refresh failure is None
        let failure_time = jwt_key_cache.last_refresh_failure.read().await;

        assert!(failure_time.is_none())
    }
}
