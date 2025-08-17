//! JWT Key Background Tasks and Refresh Operations
//!
//! This module handles background refresh tasks and implements the retry logic
//! for JWT key fetching operations. It includes:
//!
//! - Background refresh task spawning and management
//! - Exponential backoff retry implementation
//! - Cache state monitoring for proactive refreshes
//! - Failure handling and recovery strategies
//!
//! See the [module-level documentation](super) for a more detailed overview and usage.

use std::sync::atomic::Ordering;
use std::time::Instant;

use ::tokio::time::Duration;
use log::{debug, error, info};

use crate::constant::{DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_MAX_RETRIES};
use crate::error::{EsiError, OAuthError};
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Refreshes JWT keys with retry logic
    ///
    /// This method implements a blocking refresh operation with exponential backoff retry:
    /// 1. Attempts to fetch and update JWT keys from the EVE OAuth2 API
    /// 2. If initial attempt fails, retries with exponential backoff delay,
    ///    see [`DEFAULT_JWK_REFRESH_BACKOFF`] (100ms)
    /// 3. Continues retrying until success or maximum retry count is reached,
    ///    see [`DEFAULT_JWK_REFRESH_MAX_RETRIES`] (2 retries)
    /// 4. Releases the refresh lock and notifies waiting threads upon completion
    /// 5. Records refresh failures for backoff management
    ///
    /// # Implementation Details
    /// - Uses exponential backoff to gracefully handle temporary service issues
    /// - Assumes the refresh lock is already acquired before being called
    /// - Always releases the lock upon completion (success or failure)
    /// - Updates the cache on successful refresh
    /// - Records failure information for future backoff decisions
    ///
    /// # Thread Safety
    /// This method is thread-safe and designed to be called only when the refresh lock
    /// has been acquired. It properly releases the lock when done, ensuring other
    /// threads can proceed with their operations.
    ///
    /// # Returns
    /// - `Ok(`[`EveJwtKeys`]`)` if keys were successfully fetched and cached
    /// - `Err(`[`EsiError`]`)` if all retry attempts failed
    ///
    /// # Related Methods
    ///
    /// ## High-Level
    /// - [`Self::get_jwt_keys`]: Public-facing method that calls this function if
    ///   the cache is empty or expired
    ///
    /// ## Task
    /// - [`Self::trigger_background_jwt_refresh`]: Non-blocking alternative
    /// - [`Self::fetch_and_update_cache`]: Called internally to perform the actual refresh
    ///
    /// ## Cache
    /// - [`Self::cache_lock_release_and_notify`]: Used to release the lock and notify
    ///   waiting threads of the refresh completion
    pub(super) async fn refresh_jwt_keys_with_retry(&self) -> Result<EveJwtKeys, EsiError> {
        info!("Fetching fresh JWT keys");

        // We have the lock, so refresh the cache
        // Retry up to DEFAULT_JWK_REFRESH_MAX_RETRIES times with exponential backoff
        let mut retry_attempts = 0;
        let mut result = self.fetch_and_update_cache().await;

        // Retry logic - attempt retries if the initial fetch failed
        while result.is_err() && retry_attempts < DEFAULT_JWK_REFRESH_MAX_RETRIES {
            let backoff_duration = Duration::from_millis(
                // Calculate exponential backoff duration:
                // Initial backoff (DEFAULT_JWK_REFRESH_BACKOFF) multiplied by 2^retry_attempts
                // This causes wait time to double with each retry attempt
                DEFAULT_JWK_REFRESH_BACKOFF * 2u64.pow(retry_attempts as u32),
            );
            debug!(
                "JWT key fetch failed. Retrying ({}/{}) after {}ms",
                retry_attempts + 1,
                DEFAULT_JWK_REFRESH_MAX_RETRIES,
                backoff_duration.as_millis()
            );

            // Wait before retrying
            tokio::time::sleep(backoff_duration).await;

            // Try to fetch again
            result = self.fetch_and_update_cache().await;
            retry_attempts += 1;
        }

        // Always release the lock
        self.cache_lock_release_and_notify();

        // Return the result or error
        match result {
            Ok(keys) => {
                debug!("Successfully fetched and cached fresh JWT keys");
                // Clear any previous failure on success
                let mut last_failure = self.client.jwt_keys_last_refresh_failure.write().await;
                *last_failure = None;
                Ok(keys)
            }
            Err(_) => {
                let mut failure_time = self.client.jwt_keys_last_refresh_failure.write().await;
                *failure_time = Some(std::time::Instant::now());

                error!(
                    "JWT key refresh failed: attempts={}, backoff_period={}ms",
                    retry_attempts, DEFAULT_JWK_REFRESH_BACKOFF
                );

                let error_message =
                    format!("Failed to fetch JWT keys after {} attempts", retry_attempts);

                Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(
                    error_message,
                )))
            }
        }
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
    /// - `Some(`[`EveJwtKeys`]`)` if valid keys are found in the cache (may trigger refresh in background)
    /// - [`None`] if keys are not found in the cache or are expired
    ///
    /// # Related Methods
    ///
    /// ## High-Level
    /// - [`Self::get_jwt_keys`]: Public-facing method that calls this function
    ///
    /// ## Task
    /// - [`Self::trigger_background_jwt_refresh`]: Performs the actual background refresh
    ///
    /// ## Cache
    /// - [`Self::cache_lock_try_acquire`]: Attempts to acquire lock for refresh operation
    /// - [`Self::is_cache_expired`]: Determines if keys are fully expired
    ///
    /// ## Utility
    /// - [`Self::wait_for_ongoing_refresh`]: Used by other methods when refresh is in progress
    /// - [`Self::should_respect_backoff`]: Checks if we should delay refresh after failure
    /// - [`Self::is_approaching_expiry`]: Determines if keys are nearing expiration
    pub(super) async fn check_cache_and_trigger_background_refresh(&self) -> Option<EveJwtKeys> {
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
                if !should_respect_backoff && self.cache_lock_try_acquire() {
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

    /// Helper function to trigger a background JWT refresh task.
    ///
    /// This method initiates an asynchronous task to refresh the JWT keys without blocking the caller:
    /// 1. Spawns a new tokio task to perform the refresh operation
    /// 2. Fetches fresh JWT keys from EVE's OAuth2 API
    /// 3. Updates the cache with the new keys
    /// 4. Releases the refresh lock and notifies waiting threads
    /// 5. Records success or failure for backoff management
    ///
    /// # Implementation Details
    /// - Clones necessary client components to ensure thread safety
    /// - Uses tokio's task spawning to perform work asynchronously
    /// - Properly manages refresh lock state throughout the operation
    /// - Implements notifications to unblock waiting threads upon completion
    /// - Tracks refresh failures for intelligent backoff implementation
    ///
    /// # Thread Safety
    /// This method is thread-safe and designed to be called from concurrent contexts.
    /// The spawned task operates independently from the caller, ensuring non-blocking behavior
    /// while maintaining proper synchronization through atomic operations and locks.
    ///
    /// # Related Methods
    ///
    /// ## Task
    /// - [`Self::refresh_jwt_keys_with_retry`]: Alternative that blocks until completion with retries
    /// - [`Self::check_cache_and_trigger_background_refresh`]: Conditionally calls this method
    ///
    /// ## Cache
    /// - [`Self::cache_lock_try_acquire`]: Should be called before this function
    /// - [`Self::cache_lock_release_and_notify`]: Releases lock and notifies waiting threads
    pub(super) async fn trigger_background_jwt_refresh(&self) {
        debug!("Triggering background JWT refresh task");

        let esi_client = self.client;

        // Clone the required components
        let reqwest_client = esi_client.reqwest_client.clone();
        let jwt_keys_cache = esi_client.jwt_keys_cache.clone();
        let jwk_url = esi_client.jwk_url.clone();
        let refresh_in_progress = esi_client.jwt_key_refresh_in_progress.clone();
        let jwt_key_refresh_notifier = esi_client.jwt_key_refresh_notifier.clone();
        let jwt_keys_last_refresh_failure = esi_client.jwt_keys_last_refresh_failure.clone();

        tokio::spawn(async move {
            debug!("Background JWT key refresh task started");

            let result = async {
                debug!("Fetching fresh keys from JWK URL: {}", jwk_url);

                // Fetch fresh keys from EVE's OAuth2 API
                let fresh_keys = reqwest_client
                    .get(jwk_url.to_string())
                    .send()
                    .await?
                    .json::<EveJwtKeys>()
                    .await?;

                // Update the cache with the new keys
                debug!("Updating JWT keys cache");
                {
                    let mut cache = jwt_keys_cache.write().await;
                    *cache = Some((fresh_keys, Instant::now()));
                }

                debug!("JWT keys cache updated");
                Ok::<_, EsiError>(())
            }
            .await;

            // Always release the lock
            debug!("Releasing JWT key refresh lock");
            refresh_in_progress.store(false, Ordering::Release);

            // Notify waiting threads that the cache has been updated
            jwt_key_refresh_notifier.notify_waiters();

            if let Err(err) = result {
                error!("Background JWT key refresh failed: {:?}", err);

                // Record the failure time
                let mut last_failure = jwt_keys_last_refresh_failure.write().await;
                *last_failure = Some(Instant::now());
            } else {
                debug!("Background JWT key refresh task successful");

                // Clear any previous failure on success
                let mut last_failure = jwt_keys_last_refresh_failure.write().await;
                *last_failure = None;
            }
        });

        debug!("Background JWT key refresh task spawned");
    }
}
