//! # JWT Key Utility Functions
//!
//! This module provides utility functions for JWT key management, including:
//!
//! - Cache expiry calculations
//! - Backoff period enforcement
//! - Error handling and reporting
//! - Waiting mechanisms for concurrent refresh operations
//!
//! These utilities support the core JWT key operations with helper functions
//! that implement common patterns and checks used throughout the JWT key system.
//!
//! See the [module-level documentation](super) for a more detailed overview and usage.

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
    ///   [`DEFAULT_JWK_REFRESH_TIMEOUT`] seconds (5 seconds)
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
    ///   [`DEFAULT_JWK_REFRESH_TIMEOUT`] seconds (5 seconds)
    ///
    /// # Related Methods
    ///
    /// ## High-Level
    /// - [`Self::get_jwt_keys`]: Public-facing method that might trigger the waiting process
    ///
    /// ## Task
    /// - [`Self::refresh_jwt_keys_with_retry`]: Performs the actual refresh with retry logic
    /// - [`Self::check_cache_and_trigger_background_refresh`]: Used to check cache and
    ///   potentially trigger a background refresh that other threads may wait for
    ///
    /// ## Cache
    /// - [`Self::cache_lock_try_acquire`]: Used to determine if a thread should
    /// - [`Self::cache_lock_release_and_notify`]: Called by the refreshing thread to
    ///   wake up all waiting threads when the refresh operation completes
    ///   initiate a refresh or wait for another thread's refresh
    pub(super) async fn wait_for_ongoing_refresh(&self) -> Result<EveJwtKeys, EsiError> {
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
        if let Some(keys) = self.cache_get_keys().await {
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

    /// Checks if we should respect backoff period due to previous failure
    ///
    /// This method determines whether enough time has passed since the last
    /// JWT key refresh failure to attempt another refresh. It implements a
    /// simple backoff mechanism to prevent excessive API calls when the
    /// authentication service is experiencing issues.
    ///
    /// # Implementation Details
    /// - Reads from the shared [`crate::EsiClient::jwt_keys_last_refresh_failure`] timestamp
    /// - Uses [`DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF`] (60 seconds) as the minimum wait time
    ///   between refresh attempts after a failure
    ///
    /// # Thread Safety
    /// This method acquires a read lock on the failure timestamp, allowing
    /// multiple threads to check the backoff status concurrently.
    ///
    /// # Returns
    /// - `true` if we are still within the backoff period and should not attempt another refresh
    /// - `false` if either no previous failure exists or the backoff period has elapsed
    pub(super) async fn should_respect_backoff(&self) -> bool {
        match &*self.client.jwt_keys_last_refresh_failure.read().await {
            Some(last_failure) => {
                last_failure.elapsed().as_secs() < DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF
            }
            None => false,
        }
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
    ///   containing a descriptive error message regarding the error and the attempt count.
    ///
    /// # Related Methods
    ///
    /// ## Task
    /// - [`Self::refresh_jwt_keys_with_retry`]: Calls this method when refresh attempts fail
    ///
    /// ## Utility
    /// - [`Self::should_respect_backoff`]: Checks the timestamp set by this method
    pub(super) async fn record_refresh_failure(&self, attempt_count: u64) -> EsiError {
        let mut failure_time = self.client.jwt_keys_last_refresh_failure.write().await;
        *failure_time = Some(std::time::Instant::now());

        let error_message = format!("Failed to fetch JWT keys after {} attempts", attempt_count);

        error!("{}", error_message);

        EsiError::OAuthError(OAuthError::JwtKeyCacheError(error_message))
    }

    /// Determines if the cache is approaching expiry based on elapsed time
    ///
    /// Checks whether the elapsed time since the last cache update has crossed
    /// the threshold percentage of the total TTL, indicating that a proactive
    /// refresh should be triggered.
    ///
    /// # Implementation Details
    /// The threshold is defined by [`DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT`] (80%),
    /// which represents the percentage of the total TTL after which we consider
    /// the cache to be approaching expiry.
    ///
    /// # Parameters
    /// - `elapsed_seconds`: Number of seconds since the cache was last updated
    ///
    /// # Returns
    /// - `true` if the elapsed time exceeds the threshold percentage of the TTL
    /// - `false` if the cache is still well within its valid period
    pub(super) fn is_approaching_expiry(&self, elapsed_seconds: u64) -> bool {
        elapsed_seconds
            > (self.client.jwt_keys_cache_ttl * DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT
                / 100)
    }

    /// Determines if the cache has completely expired based on elapsed time
    ///
    /// Checks if the elapsed time since the last cache update has reached or
    /// exceeded the configured TTL (default: 3600 seconds/1 hour), indicating
    /// that the cached keys should no longer be used.
    ///
    /// # Parameters
    /// - `elapsed_seconds`: Number of seconds since the cache was last updated
    ///
    /// # Returns
    /// - `true` if the elapsed time has reached or exceeded the TTL
    /// - `false` if the cache is still within its valid period
    ///
    /// # Related Methods
    ///
    /// ## Utility
    /// - [`Self::is_approaching_expiry`]: Checks if the cache is nearing expiration
    ///   but hasn't fully expired yet
    pub(super) fn is_cache_expired(&self, elapsed_seconds: u64) -> bool {
        elapsed_seconds >= self.client.jwt_keys_cache_ttl
    }
}
