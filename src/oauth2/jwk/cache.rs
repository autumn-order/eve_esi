//! OAuth2 JWT key cache
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

use std::time::Instant;
use std::{sync::atomic::AtomicBool, time::Duration};

use log::{debug, info, trace};
use tokio::sync::{Notify, RwLock};

use crate::{
    config::Config,
    constant::{
        DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT, DEFAULT_JWK_CACHE_TTL,
        DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_COOLDOWN, DEFAULT_JWK_REFRESH_MAX_RETRIES,
        DEFAULT_JWK_REFRESH_TIMEOUT, DEFAULT_JWK_URL,
    },
    model::oauth2::EveJwtKeys,
};

/// Configuration for JWT key caching and refreshing
///
/// Provides fields which determine the JWT key cache TTL, the link JWT keys are fetched from,
/// the logic behind refresh attempts, and settings the proactive JWT key cache background refresh.
///
/// For an overview regarding the JWT key cache, see the [module-level documentation](self)
#[derive(Clone)]
pub(crate) struct JwtKeyCacheConfig {
    // Cache Settings
    /// JWT key cache lifetime before expiration (3600 seconds representing 1 hour)
    pub(crate) cache_ttl: Duration,

    // Refresh Settings
    /// JSON web token key URL that provides keys used to validate tokens
    pub(crate) jwk_url: String,
    /// Backoff period after a JWT key refresh failure when cache is empty or expired (default 100 milliseconds)
    pub(crate) refresh_backoff: Duration,
    /// Timeout when waiting for another thread to refresh JWT key (default 5 seconds)
    pub(crate) refresh_timeout: Duration,
    /// Cooldown period after a failed set of JWT key refresh attempts (default 60 seconds)
    pub(crate) refresh_cooldown: Duration,
    /// Maximum number of retries for JWT key refresh when cache is empty or expired (default 2 retries)
    pub(crate) refresh_max_retries: u32,

    // Background Refresh Settings
    /// Determines whether or not a background task is spawned to refresh JWT keys proactively when cache is nearing expiration
    pub(crate) background_refresh_enabled: bool,
    /// Percentage of jwk_cache_ttl for when the background JWT key refresh is triggered (default 80%)
    pub(crate) background_refresh_threshold: u64,
}

/// JWT key cache for caching keys & coordinating refreshes
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
pub(crate) struct JwtKeyCache {
    /// RwLock with a tuple containing JWT keys and timestamp of when keys were updated
    pub(super) cache: RwLock<Option<(EveJwtKeys, Instant)>>,
    /// AtomicBool indicating whether a JWT key refresh is currently in progress
    pub(super) refresh_lock: AtomicBool,
    /// Notifier for when a JWT key refresh is completed
    pub(super) refresh_notifier: Notify,
    /// RwLock with a timestamp of last failed set of JWT key refresh attemmpts
    pub(super) last_refresh_failure: RwLock<Option<Instant>>,
    /// Configuration for JWT key cache & refreshes
    pub(super) config: JwtKeyCacheConfig,
}

impl JwtKeyCacheConfig {
    /// Initializes a new JWT key cache config with the default settings
    pub(crate) fn new() -> Self {
        Self {
            // Cache Settings
            cache_ttl: DEFAULT_JWK_CACHE_TTL,

            // Refresh Settings
            jwk_url: DEFAULT_JWK_URL.to_string(),
            refresh_max_retries: DEFAULT_JWK_REFRESH_MAX_RETRIES,
            refresh_backoff: DEFAULT_JWK_REFRESH_BACKOFF,
            refresh_timeout: DEFAULT_JWK_REFRESH_TIMEOUT,
            refresh_cooldown: DEFAULT_JWK_REFRESH_COOLDOWN,

            // Background Refresh Settings
            background_refresh_enabled: true,
            background_refresh_threshold: DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
        }
    }
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
    pub(crate) fn new(config: &Config) -> Self {
        Self {
            cache: RwLock::new(None),
            refresh_lock: AtomicBool::new(false),
            refresh_notifier: Notify::new(),
            last_refresh_failure: RwLock::new(None),
            config: config.jwt_key_cache_config.clone(),
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
        trace!("Attempting to retrieve JWT keys from cache");

        // Retrieve the cache
        let cache = self.cache.read().await;

        // Check if the cache has keys stored
        if let Some((keys, timestamp)) = &*cache {
            let elapsed = timestamp.elapsed().as_secs();

            debug!(
                "Found JWT keys in cache: key_count={}, age={}s",
                keys.keys.len(),
                elapsed
            );

            // Return the keys found in cache
            return Some((keys.clone(), timestamp.clone()));
        }

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
        let key_count = keys.keys.len();

        let mut cache = self.cache.write().await;
        *cache = Some((keys, std::time::Instant::now()));

        debug!(
            "JWT keys cache successfully updated with {} keys",
            &key_count
        );
    }

    /// Clears the JWT key cache of any keys present
    ///
    /// You would typically use this in the event of a validation failure
    /// indicating that the JWT keys currently in the cache are out of date
    /// or are malformed.
    ///
    /// The cache will not clear if the keys it contains were set within the 60 second
    /// refresh cooldown period (default). This is intended in case worst case scenario
    /// this function is called repetitively. This prevents refresh attempts from occurring back
    /// to back which are triggered when [`super::JwkApi::get_jwt_keys`] is called and the cache
    /// is expired or empty.
    ///
    /// # Thread Safety
    /// This method acquires a write lock before checking how recently the keys were updated to
    /// avoid accidentally overwriting an update immediately after it occurs. The write lock is
    /// then released once the cache is cleared or it is determined the cache is not yet eligible
    /// to clear due to keys being set recently.
    ///
    /// # Returns
    /// - [`bool`]: Indicates whether or not the cache was cleared.
    pub(crate) async fn clear_cache(self: &Self) -> bool {
        debug!("Attempting to clear JWT key cache");

        // Acquire write lock first to not accidentally overwrite any updates
        let mut cache = self.cache.write().await;

        // Ensure keys aren't recently updated
        if let Some((_, timestamp)) = &*cache {
            // If keys are older than 60 second refresh cooldown period (default) clear cache
            let sixty_seconds_ago = Instant::now() - self.config.refresh_cooldown;

            if timestamp < &sixty_seconds_ago {
                // Clear the cache
                let elapsed = timestamp.elapsed().as_secs();
                info!(
                    "Clearing JWT key cache of keys that were set {}s ago",
                    elapsed
                );

                *cache = None;

                true
            } else {
                debug!(
                    "JWT key cache not cleared due to keys being within {} seconds of age",
                    self.config.refresh_cooldown.as_secs()
                );

                false
            }
        } else {
            debug!("JWT key cache is currently empty, no need to clear it.");

            false
        }
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
            debug!("Successfully acquired JWT key refresh lock");

            // Lock successfully acquired
            true
        } else {
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
        self.refresh_lock
            .store(false, std::sync::atomic::Ordering::Release);

        // Notify waiters
        self.refresh_notifier.notify_waiters();

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
    use crate::Client;

    use super::super::tests::create_mock_keys;

    /// Validates function returns Some keys when cache has keys
    ///
    /// Checks that when the cache has keys set, the cache_get_keys
    /// function returns them properly without issues.
    ///
    /// # Test Setup
    /// - Setup basic Client
    /// - Set Client JWT key cache with mock keys
    ///
    /// # Assertions
    /// - Verify function returns Some(EveJwtKeys)
    #[tokio::test]
    async fn test_cache_get_keys_some() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Set JWT key cache
        {
            let keys = (create_mock_keys(), std::time::Instant::now());

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
    /// - Setup basic Client
    /// - Do not set the JWT key cache
    ///
    /// # Assertions
    /// - Verify function returns None
    #[tokio::test]
    async fn test_cache_get_keys_none() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Do not set JWT key cache which is None by default

        // Test function
        let result = jwt_key_cache.get_keys().await;

        // Assert None
        assert!(result.is_none())
    }
}

#[cfg(test)]
mod cache_update_keys_tests {
    use crate::Client;

    use super::super::tests::create_mock_keys;

    /// Validates that cache properly updates with new keys
    ///
    /// Checks that writing new keys to the JWT key cache on
    /// Client is successful.
    ///
    /// # Test Setup
    /// - Setup basic Client
    /// - Create mock JWT keys
    ///
    /// # Assertions
    /// - Assert that the Client jwt_keys_cache now is Some()
    #[tokio::test]
    async fn test_cache_update_keys() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Create mock keys
        let mock_keys = create_mock_keys();

        // Test function
        jwt_key_cache.update_keys(mock_keys).await;

        // Assert some
        let cache = jwt_key_cache.cache.read().await;
        let result = &*cache;

        assert!(result.is_some())
    }
}

#[cfg(test)]
mod clear_cache_tests {
    use std::time::{Duration, Instant};

    use super::super::tests::create_mock_keys;
    use crate::tests::setup;

    /// Cache successfully cleared
    ///
    /// Cache will clear so long as keys present are older than 60 seconds.
    ///
    /// # Test Setup
    /// - Setup a basic ESI client
    /// - Fill JWT key cache with mock keys older than 60 seconds
    ///
    /// # Assert
    /// - Assert attempt was made to clear the cache
    /// - Assert cache is now empty
    #[tokio::test]
    async fn cache_clear_success() {
        // Setup a basic ESI client
        let (esi_client, _) = setup().await;

        // Fill JWT key cache with mock keys older than 60 seconds
        {
            // Create timestamp older than 60 seconds
            let mock_keys = create_mock_keys();
            let timestamp = Instant::now() - Duration::from_secs(61);

            // Acquire write lock & set cache
            let mut cache = esi_client.inner.jwt_key_cache.cache.write().await;
            *cache = Some((mock_keys, timestamp))
        } // Write lock released here

        // Clear the JWT key cache
        let cache_cleared = esi_client.inner.jwt_key_cache.clear_cache().await;

        // Assert attempt was made to clear the cache
        assert_eq!(cache_cleared, true);

        // Assert cache is now empty
        let cache = esi_client.inner.jwt_key_cache.get_keys().await;

        assert!(cache.is_none())
    }

    /// Cache doesn't clear because keys are recent
    ///
    /// # Test Setup
    /// - Setup a basic ESI client
    /// - Fill JWT key cache with mock keys
    /// - Acquire a refresh lock to indicate a refresh is ongoing
    ///
    /// # Assert
    /// - Assert refresh lock is in place
    /// - Assert no attempt was made to clear the cache
    /// - Assert cache has not been cleared
    #[tokio::test]
    async fn cache_clear_recent_keys() {
        // Setup a basic ESI client
        let (esi_client, _) = setup().await;

        // Fill JWT key cache with recent mock keys
        let mock_keys = create_mock_keys();

        esi_client.inner.jwt_key_cache.update_keys(mock_keys).await;

        // Acquire a refresh lock
        let lock_acquired = esi_client.inner.jwt_key_cache.refresh_lock_try_acquire();

        // Assert refresh lock is in place
        assert_eq!(lock_acquired, true);

        // Attempt to clear the JWT key cache
        let cache_cleared = esi_client.inner.jwt_key_cache.clear_cache().await;

        // Assert no attempt was made to clear the cache
        assert_eq!(cache_cleared, false);

        // Assert cache has not been cleared
        let cache = esi_client.inner.jwt_key_cache.get_keys().await;

        assert!(cache.is_some())
    }
}

#[cfg(test)]
mod jwk_refresh_lock_try_acquire_tests {
    use crate::Client;

    /// Checks that lock is properly acquired when not already in use
    ///
    /// Attempts to acquire a lock to refresh JWT keys on a new
    /// Client which should return as successful (true) indicating
    /// that no other threads are currently attempting a key refresh.
    ///
    /// # Test Setup
    /// - Setup a basic Client
    /// - Attempt to acquire a lock for JWT key refresh
    ///
    /// # Assertions
    /// - Assert that result is true when acquiring lock
    #[test]
    fn test_jwk_refresh_lock_try_acquire_success() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        // Attempt to acquire lock
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

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
    /// - Setup basic Client
    /// - Acquire a lock initially
    /// - Attempt to acquire lock again
    ///
    /// # Assertions
    /// - Asserts that result is false when attempting to acquire lock
    ///   a second time indicating it is already in use.
    #[test]
    fn test_jwk_refresh_lock_try_acquire_unsuccessful() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        // Acquire a lock initially
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        let lock = jwt_key_cache.refresh_lock_try_acquire();

        if !lock {
            panic!("Failed to acquire initial lock")
        }

        // Acquire lock a second time
        // Should return false indicating lock is already in use
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        let lock_acquired = jwt_key_cache.refresh_lock_try_acquire();

        // Assert
        assert_eq!(lock_acquired, false)
    }
}

#[cfg(test)]
mod jwk_lock_release_and_notify_tests {
    use std::time::Duration;

    use crate::Client;

    /// Verifies that lock is successfully released & waiters are notified
    ///
    /// Acquires a lock and sets up a notification listener which listens
    /// for the notification of when the lock is released. If the notification
    /// is never received than the listener will timeout. Checks to ensure that
    /// notification was properly received as well as the lock was released
    /// without issues.
    ///
    /// # Test Setup
    /// - Create a basic Client
    /// - Acquire a JWT key refresh lock
    /// - Setup a notification listener
    ///
    /// # Assertions
    /// - Assert that lock has been properly acquired
    /// - Assert that lock release notification was received
    /// - Assert that lock has been properly released
    #[tokio::test]
    async fn test_jwk_refresh_lock_release_and_notify_success() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        // Acquire a lock
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

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
    use crate::Client;

    /// Set the last refresh failure timestamp
    ///
    /// # Test Setup
    /// - Create a basic Client
    ///
    /// # Assertions
    /// - Assert last refresh failure is Some
    /// - Assert failure time matches timestamp set
    #[tokio::test]
    async fn test_set_refresh_failure_some() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

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
    /// - Create a basic Client
    /// - Set a refresh failure timestamp
    ///
    /// # Assertions
    /// - Assert last refresh failure is None
    #[tokio::test]
    async fn test_set_refresh_failure_none() {
        // Setup basic Client
        let esi_client = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

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
