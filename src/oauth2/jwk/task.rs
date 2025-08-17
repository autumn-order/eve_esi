//! # JWT Key Background Tasks and Refresh Operations
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
use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Refreshes JWT keys with retry logic
    /// This function assumes the refresh lock is already acquired
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
                // Record the failure with retry count + 1 (to account for initial attempt)
                let error = self
                    .record_refresh_failure(DEFAULT_JWK_REFRESH_MAX_RETRIES + 1)
                    .await;
                Err(error)
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
