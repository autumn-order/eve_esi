use log::{debug, error};
use tokio::time::Duration;

use crate::constant::{
    DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF, DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
    DEFAULT_JWK_REFRESH_TIMEOUT,
};
use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::error::OAuthError;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Waits for an ongoing JWT key cache refresh operation to complete and returns the result
    ///
    /// This method is designed to be called when a thread detects that another thread
    /// is already refreshing the JWT keys. Instead of initiating another refresh or failing
    /// immediately, this method allows the current thread to efficiently wait for the
    /// completion of the ongoing refresh operation.
    ///
    /// # Implementation Details
    /// - Uses the async notification pattern via [`tokio::sync::Notify`]
    /// - Waits for either a notification from the refreshing thread or times out after
    ///   [`DEFAULT_JWK_REFRESH_TIMEOUT`] seconds
    /// - After the wait completes (either via notification or timeout), attempts to
    ///   retrieve the keys from the cache one more time
    /// - If keys are still not available after waiting, returns a descriptive error
    ///
    /// # Thread Safety
    /// This method is thread-safe and can be called concurrently by multiple threads.
    /// All threads will be notified when the refresh completes, ensuring efficient
    /// wake-up without unnecessary polling or lock contention.
    ///
    /// # Returns
    /// - Ok([`EveJwtKeys`]) if the refresh was successful and keys are now in the cache
    /// - Err([`EsiError`]) if the refresh attempt failed or timed out after
    ///   [`DEFAULT_JWK_REFRESH_TIMEOUT`] seconds (typically 5 seconds)
    ///
    /// # Related Methods
    /// - [`Self::release_refresh_lock_and_notify`]: Called by the refreshing thread to
    ///   wake up all waiting threads when the refresh operation completes
    /// - [`Self::check_cache_and_trigger_background_refresh`]: Used to check cache and
    ///   potentially trigger a background refresh that other threads may wait for
    /// - [`Self::try_acquire_refresh_lock`]: Used to determine if a thread should
    ///   initiate a refresh or wait for another thread's refresh
    /// - [`Self::refresh_jwt_keys_with_retry`]: Performs the actual refresh with retry logic
    /// - [`Self::get_jwt_keys`]: Public-facing method that might trigger the waiting process
    pub async fn wait_for_ongoing_refresh(&self) -> Result<EveJwtKeys, EsiError> {
        debug!("Waiting for another thread to refresh JWT keys");

        // Create a future that waits for the notification
        let notify_future = self.client.jwt_key_refresh_notifier.notified();

        // Wait for the notification or a timeout (as fallback)
        tokio::select! {
            _ = notify_future => {
                debug!("Received notification that JWT keys refresh is complete");
            }
            _ = tokio::time::sleep(Duration::from_secs(DEFAULT_JWK_REFRESH_TIMEOUT)) => {
                debug!("Timed out waiting for JWT keys refresh notification");
            }
        }

        // Try cache again after being notified
        if let Some(keys) = self.get_keys_from_cache().await {
            debug!("Successfully retrieved JWT keys after waiting for refresh");
            return Ok(keys);
        }

        // Create a descriptive error message
        let error_message =
            "Failed to retrieve JWT keys from cache after waiting for refresh".to_string();

        // Log the error at debug level
        debug!("{}", error_message);

        // Return appropriate error type
        Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(
            error_message,
        )))
    }

    /// Checks if the cache has valid keys and triggers background refresh if needed
    ///
    /// This method implements a multi-step process to efficiently manage the JWT key cache:
    /// 1. Attempts to retrieve JWT keys from the cache
    /// 2. If keys are found but approaching expiry, conditionally triggers a background refresh
    /// 3. Returns the cached keys if they're not fully expired, even if a refresh was triggered
    ///
    /// The background refresh is only triggered when ALL of the following conditions are met:
    /// - The cached keys are approaching their expiry (but not yet expired)
    /// - No refresh operation is currently in progress (acquired via atomic lock)
    /// - Not within the backoff period from a previous failed refresh
    ///
    /// # Implementation Details
    /// - Uses a read lock on the cache to check current state without blocking other readers
    /// - Uses atomic operations to safely check and set the refresh-in-progress flag
    /// - Implements the "refresh ahead" pattern to update cache before expiry
    /// - Returns keys even while triggering refresh to prevent client blocking
    /// - Falls back to returning None if keys are fully expired or not in cache
    ///
    /// # Thread Safety
    /// This method is thread-safe and can be called concurrently by multiple threads.
    /// It uses appropriate locking to ensure consistency when reading the cache while
    /// preventing multiple simultaneous refresh operations.
    ///
    /// # Returns
    /// - Some([`EveJwtKeys`]) if valid keys are found in the cache (may trigger refresh in background)
    /// - [`None`] if keys are not found in the cache or are expired
    ///
    /// # Related Methods
    /// - [`Self::is_approaching_expiry`]: Determines if keys are nearing expiration
    /// - [`Self::is_cache_expired`]: Determines if keys are fully expired
    /// - [`Self::should_respect_backoff`]: Checks if we should delay refresh after failure
    /// - [`Self::try_acquire_refresh_lock`]: Attempts to acquire lock for refresh operation
    /// - [`Self::trigger_background_jwt_refresh`]: Performs the actual background refresh
    /// - [`Self::wait_for_ongoing_refresh`]: Used by other methods when refresh is in progress
    /// - [`Self::get_jwt_keys`]: Public-facing method that calls this function
    pub async fn check_cache_and_trigger_background_refresh(&self) -> Option<EveJwtKeys> {
        // Retrieve keys from cache
        let keys = {
            let cache = self.client.jwt_keys_cache.read().await;
            match &*cache {
                Some((keys, timestamp)) => Some((keys.clone(), *timestamp)),
                None => None,
            }
        };

        if let Some((keys, timestamp)) = keys {
            debug!("JWT keys found in cache");

            // Check if we should run a background refresh task
            let is_approaching_expiry = self.is_approaching_expiry(timestamp.elapsed().as_secs());

            if is_approaching_expiry {
                // Check if we should respect a backoff period due to previous failure
                let should_respect_backoff = self.should_respect_backoff().await;

                // Only trigger background refresh if not in backoff period and we can acquire the lock
                if !should_respect_backoff && self.try_acquire_refresh_lock() {
                    self.trigger_background_jwt_refresh().await;
                }
            }

            // Return keys if cache is not expired
            if !self.is_cache_expired(timestamp.elapsed().as_secs()) {
                debug!("Using cached JWT keys");
                return Some(keys);
            } else {
                debug!("JWT keys cache expired");
            }
        } else {
            debug!("JWT keys cache miss");
        }

        None
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
    /// - [`Self::release_refresh_lock_and_notify`]: Releases the lock acquired by this method
    /// - [`Self::refresh_jwt_keys_with_retry`]: Uses this method to ensure exclusive refresh access
    /// - [`Self::get_jwt_keys`]: Public-facing method that needs exclusive refresh access
    pub fn try_acquire_refresh_lock(&self) -> bool {
        !self
            .client
            .jwt_key_refresh_in_progress
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
    }

    /// Retrieves JWT keys directly from cache without validation or refresh attempts
    ///
    /// This is a low-level utility method that provides direct access to the JWT keys
    /// stored in the cache. Unlike higher-level methods such as
    /// `check_cache_and_trigger_background_refresh`, this method:
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
    /// - [`None`] if the cache is empty (no keys have been fetched yet)
    ///
    /// # Related Methods
    ///
    /// - [`Self::check_cache_and_trigger_background_refresh`]: Higher-level method that checks
    ///   expiration and may trigger background refresh
    /// - [`Self::wait_for_ongoing_refresh`]: Used after detecting an ongoing refresh operation
    /// - [`Self::is_cache_expired`]: Can be used alongside this method to check validity
    /// - [`Self::fetch_and_update_cache`]: Updates the cache that this method reads from
    /// - [`Self::get_jwt_keys`]: Public-facing method that uses this utility method
    pub async fn get_keys_from_cache(&self) -> Option<EveJwtKeys> {
        let cache = self.client.jwt_keys_cache.read().await;
        match &*cache {
            Some((keys, _)) => Some(keys.clone()),
            None => None,
        }
    }

    /// Determines if the cache is approaching expiry based on elapsed time
    ///
    /// Checks whether the elapsed time since the last cache update has crossed
    /// the threshold percentage of the total TTL, indicating that a proactive
    /// refresh should be triggered.
    ///
    /// # Implementation Details
    /// The threshold is defined by [`DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT`],
    /// which represents the percentage of the total TTL after which we consider
    /// the cache to be approaching expiry.
    ///
    /// # Parameters
    /// - `elapsed_seconds`: Number of seconds since the cache was last updated
    ///
    /// # Returns
    /// - `true` if the elapsed time exceeds the threshold percentage of the TTL
    /// - `false` if the cache is still well within its valid period
    pub fn is_approaching_expiry(&self, elapsed_seconds: u64) -> bool {
        elapsed_seconds
            > (self.client.jwt_keys_cache_ttl * DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT
                / 100)
    }

    /// Determines if the cache has completely expired based on elapsed time
    ///
    /// Checks if the elapsed time since the last cache update has reached or
    /// exceeded the configured TTL, indicating that the cached keys should no
    /// longer be used.
    ///
    /// # Parameters
    /// - `elapsed_seconds`: Number of seconds since the cache was last updated
    ///
    /// # Returns
    /// - `true` if the elapsed time has reached or exceeded the TTL
    /// - `false` if the cache is still within its valid period
    ///
    /// # Related Methods
    /// - [`Self::is_approaching_expiry`]: Checks if the cache is nearing expiration
    ///   but hasn't fully expired yet
    pub fn is_cache_expired(&self, elapsed_seconds: u64) -> bool {
        elapsed_seconds >= self.client.jwt_keys_cache_ttl
    }

    /// Checks if we should respect backoff period due to previous failure
    ///
    /// This method determines whether enough time has passed since the last
    /// JWT key refresh failure to attempt another refresh. It implements a
    /// simple backoff mechanism to prevent excessive API calls when the
    /// authentication service is experiencing issues.
    ///
    /// # Implementation Details
    /// - Reads from the shared [`crate::EsiClient::jwt_keys_last_refresh_failure`] timestamp
    /// - Uses [`DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF`] as the minimum wait time
    ///   between refresh attempts after a failure
    ///
    /// # Thread Safety
    /// This method acquires a read lock on the failure timestamp, allowing
    /// multiple threads to check the backoff status concurrently.
    ///
    /// # Returns
    /// - `true` if we are still within the backoff period and should not attempt another refresh
    /// - `false` if either no previous failure exists or the backoff period has elapsed
    pub async fn should_respect_backoff(&self) -> bool {
        match &*self.client.jwt_keys_last_refresh_failure.read().await {
            Some(last_failure) => {
                last_failure.elapsed().as_secs() < DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF
            }
            None => false,
        }
    }

    /// Updates the JWT keys cache with new keys and the current timestamp
    ///
    /// Stores the provided JWT keys in the cache along with the current timestamp,
    /// which will be used to determine when the keys should be refreshed next.
    ///
    /// # Implementation Details
    /// - Acquires a write lock on the JWT keys cache
    /// - Stores the keys along with the current timestamp as an `Instant`
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
    /// - [`Self::fetch_and_update_cache`]: Uses this method to update the cache with freshly fetched keys
    /// - [`Self::get_jwt_keys`]: Public-facing method that relies on this method for cache updates
    pub async fn update_jwt_keys_cache(&self, keys: EveJwtKeys) {
        let mut cache = self.client.jwt_keys_cache.write().await;
        *cache = Some((keys, std::time::Instant::now()));
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
    /// - [`Self::try_acquire_refresh_lock`]: The counterpart method used to acquire the lock
    /// - [`Self::wait_for_ongoing_refresh`]: Method used by threads waiting for notification
    /// - [`Self::refresh_jwt_keys_with_retry`]: Calls this method when refresh operations complete
    /// - [`Self::trigger_background_jwt_refresh`]: Calls this method in background tasks
    pub fn release_refresh_lock_and_notify(&self) {
        debug!("Releasing JWT key refresh lock");
        self.client
            .jwt_key_refresh_in_progress
            .store(false, std::sync::atomic::Ordering::Release);

        // Notify waiters
        self.client.jwt_key_refresh_notifier.notify_waiters();
    }

    /// Records a JWT key refresh failure and returns an appropriate error
    ///
    /// This method is called when a JWT key refresh operation fails after
    /// multiple attempts. It records the current time as the last failure
    /// timestamp, which is used by the backoff mechanism to prevent excessive
    /// retry attempts.
    ///
    /// # Implementation Details
    /// - Updates the [`crate::EsiClient::jwt_keys_last_refresh_failure`] timestamp
    ///   with the current time
    /// - Creates a descriptive error message that includes the attempt count
    /// - Logs the error at the ERROR level
    /// - Constructs and returns an appropriate [`EsiError`] instance
    ///
    /// # Thread Safety
    /// This method acquires a write lock on the failure timestamp, ensuring that
    /// no other thread can read or write to it while the update is in progress.
    ///
    /// # Parameters
    /// - `attempt_count`: The number of attempts that were made before giving up
    ///
    /// # Returns
    /// - An [`EsiError::OAuthError`] with a [`OAuthError::JwtKeyCacheError`] variant
    ///   containing a descriptive error message
    ///
    /// # Related Methods
    /// - [`Self::refresh_jwt_keys_with_retry`]: Calls this method when refresh attempts fail
    /// - [`Self::should_respect_backoff`]: Checks the timestamp set by this method
    pub async fn record_refresh_failure(&self, attempt_count: u64) -> EsiError {
        let mut failure_time = self.client.jwt_keys_last_refresh_failure.write().await;
        *failure_time = Some(std::time::Instant::now());

        let error_message = format!("Failed to fetch JWT keys after {} attempts", attempt_count);

        error!("{}", error_message);

        EsiError::OAuthError(OAuthError::JwtKeyCacheError(error_message))
    }
}
