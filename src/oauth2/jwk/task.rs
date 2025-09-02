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

use std::time::Instant;

use ::tokio::time::Duration;
use log::{debug, error, info, trace, warn};

use crate::error::{EsiError, OAuthError};
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

use super::jwk::fetch_jwt_keys;
use super::util::check_refresh_cooldown;
use super::util_cache::cache_get_keys;
use super::util_refresh::{jwk_refresh_lock_release_and_notify, jwk_refresh_lock_try_acquire};

impl<'a> OAuth2Api<'a> {
    /// Refreshes JWT keys with retry logic
    ///
    /// This method implements a blocking refresh operation with exponential backoff retry:
    /// 1. Attempts to fetch and update JWT keys from the EVE OAuth2 API
    /// 2. If initial attempt fails, retries with exponential backoff delay defined by the
    ///    [`OAuthConfig::jwk_refresh_backoff`](crate::oauth2::OAuth2Config::jwk_refresh_backoff)
    ///    field used by the [`EsiClient`](crate::EsiClient). By default this is 100ms.
    /// 3. Continues retrying until success or maximum retry count is reached defined by the
    ///    [`OAuthConfig::jwk_refresh_max_retries`](crate::oauth2::OAuth2Config::jwk_refresh_max_retries)
    ///    field used by the [`EsiClient`](crate::EsiClient). By default this is 2 retries.
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
    pub(super) async fn refresh_jwt_keys_with_retry(&self) -> Result<EveJwtKeys, EsiError> {
        let esi_client = self.client;
        let oauth2_config = &esi_client.oauth2_config;
        let jwt_key_cache = &esi_client.jwt_key_cache;

        let max_retries = oauth2_config.jwk_refresh_max_retries;
        let refresh_backoff = oauth2_config.jwk_refresh_backoff;
        let last_refresh_failure = &jwt_key_cache.last_refresh_failure;

        #[cfg(not(tarpaulin_include))]
        info!("Starting JWT keys refresh operation");

        // Track operation timing for performance monitoring
        let start_time = std::time::Instant::now();

        // We have the lock, so refresh the cache
        // Retry up to DEFAULT_JWK_REFRESH_MAX_RETRIES times with exponential backoff
        let mut retry_attempts = 0;

        #[cfg(not(tarpaulin_include))]
        debug!("Attempting initial JWT key fetch");

        let mut result = self.fetch_and_update_cache().await;

        // Retry logic - attempt retries if the initial fetch failed
        while result.is_err() && retry_attempts < max_retries {
            let backoff_duration = Duration::from_millis(
                // Calculate exponential backoff duration:
                // Initial backoff (100ms default) multiplied by 2^retry_attempts
                // This causes wait time to double with each retry attempt
                refresh_backoff * 2u64.pow(retry_attempts as u32),
            );

            #[cfg(not(tarpaulin_include))]
            debug!(
                "JWT key fetch failed. Retrying ({}/{}) after {}ms",
                retry_attempts + 1,
                max_retries,
                backoff_duration.as_millis()
            );

            // Wait before retrying
            tokio::time::sleep(backoff_duration).await;

            // Try to fetch again
            #[cfg(not(tarpaulin_include))]
            debug!(
                "Retry attempt # {}: fetching JWT keys after backoff",
                retry_attempts + 1
            );

            result = self.fetch_and_update_cache().await;
            retry_attempts += 1;
        }

        // Always release the lock
        jwk_refresh_lock_release_and_notify(
            &jwt_key_cache.refresh_lock,
            &jwt_key_cache.refresh_notifier,
        );

        // Return the result or error
        match result {
            Ok(keys) => {
                let elapsed = start_time.elapsed();

                #[cfg(not(tarpaulin_include))]
                info!(
                    "Successfully fetched and cached fresh JWT keys (took {}ms)",
                    elapsed.as_millis()
                );

                #[cfg(not(tarpaulin_include))]
                debug!("JWT keys cache refreshed with {} keys", keys.keys.len());

                // Clear any previous failure on success
                let mut last_failure = last_refresh_failure.write().await;
                *last_failure = None;

                Ok(keys)
            }
            Err(err) => {
                let elapsed = start_time.elapsed();
                let mut failure_time = last_refresh_failure.write().await;
                *failure_time = Some(std::time::Instant::now());

                #[cfg(not(tarpaulin_include))]
                error!("JWT key refresh failed after {}ms: attempts={}, backoff_period={}ms, error={:?}",
                    elapsed.as_millis(), retry_attempts, refresh_backoff, err);

                // Return Error of type EsiError::ReqwestError
                Err(err.into())
            }
        }
    }

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
    ///   the timeout defined by the [`OAuthConfig::jwk_refresh_timeout`](crate::oauth2::OAuth2Config::jwk_refresh_timeout)
    ///   field used by the [`EsiClient`](crate::EsiClient). By default this is 5 seconds.
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
    pub(super) async fn wait_for_ongoing_refresh(&self) -> Result<EveJwtKeys, EsiError> {
        let esi_client = self.client;
        let jwt_key_cache = &esi_client.jwt_key_cache;

        let jwk_refresh_timeout = esi_client.oauth2_config.jwk_refresh_timeout;

        let start_time = Instant::now();

        #[cfg(not(tarpaulin_include))]
        debug!("Waiting for another thread to refresh JWT keys");

        // Create a future that waits for the notification
        let notify_future = jwt_key_cache.refresh_notifier.notified();

        #[cfg(not(tarpaulin_include))]
        trace!("Created notification future for JWT key refresh wait");

        let refresh_timeout = Duration::from_secs(jwk_refresh_timeout);
        let refresh_success = tokio::select! {
            _ = notify_future => {true}
            _ = tokio::time::sleep(refresh_timeout) => {false}
        };

        // Return an error if the refresh timed out
        let elapsed = start_time.elapsed();
        if !refresh_success {
            let error_message = format!(
                "Timed out after waiting {}ms for JWT key refresh.",
                elapsed.as_millis()
            );

            #[cfg(not(tarpaulin_include))]
            debug!("{}", error_message);

            // Return error indicating function timed out waiting JWT key refresh
            return Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(
                error_message,
            )));
        }

        // Attempt to retrieve keys from cache
        if let Some((keys, _)) = cache_get_keys(&esi_client.jwt_key_cache).await {
            #[cfg(not(tarpaulin_include))]
            debug!(
                "Successfully retrieved JWT keys from cache after waiting {}ms for refresh",
                elapsed.as_millis()
            );

            // Return keys if successfully retrieved from cache
            return Ok(keys);
        }

        // If the refresh request failed then no keys will be found in the cache
        let error_message = format!(
            "Failed to retrieve JWT keys from cache after waiting {}ms for refresh.
                    Likely due to a failure to refresh the keys.",
            elapsed.as_millis()
        );

        #[cfg(not(tarpaulin_include))]
        debug!("{}", error_message);

        // Return an error indicating no keys were found in cache
        Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(
            error_message,
        )))
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
    /// The background refresh is only triggered when ALL of the following conditions are met:
    /// - Not within the backoff period from a previous failed refresh
    /// - No refresh operation is currently in progress (acquired via atomic lock)
    ///
    /// # Implementation Details
    /// - Uses atomic operations to safely check and set the refresh-in-progress flag
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
    /// # Returns
    /// - `bool` indicating whether or not a background refresh was triggered
    pub(super) async fn trigger_background_jwt_refresh(&self) -> bool {
        let esi_client = self.client;
        let jwt_key_cache = &esi_client.jwt_key_cache;

        let jwk_refresh_cooldown = esi_client.oauth2_config.jwk_refresh_cooldown;
        let last_refresh_failure = &jwt_key_cache.last_refresh_failure;

        // Check if we are still in cooldown due to fetch failure within cooldown period
        if check_refresh_cooldown(jwk_refresh_cooldown, last_refresh_failure)
            .await
            .is_some()
        {
            #[cfg(not(tarpaulin_include))]
            debug!("Respecting refresh cooldown, delaying JWT key refresh");

            return false;
        }

        // Attempt to acquire a lock to perform the refresh
        if !jwk_refresh_lock_try_acquire(&jwt_key_cache.refresh_lock) {
            #[cfg(not(tarpaulin_include))]
            debug!("JWT key refresh already in progress");

            return false;
        }

        #[cfg(not(tarpaulin_include))]
        debug!("Triggering background JWT refresh task");

        // Clone the required components
        let reqwest_client = esi_client.reqwest_client.clone();
        let jwt_key_cache = esi_client.jwt_key_cache.clone();
        let jwk_url = esi_client.oauth2_config.jwk_url.clone();

        #[cfg(not(tarpaulin_include))]
        debug!(
            "Preparing background refresh task with JWK URL: {}",
            jwk_url
        );

        // Ignore tarpaulin code coverage reports on this function
        //
        // There are a significant amount of inaccuracies in tarpualin reporting
        // on this function, namely it keeps reporting `#[cfg(not(tarpaulin_include))]` itself
        // as an uncovered line. This may be related to the usage of `tokio::spawn`.
        //
        // It may be possible to refactor so that this code segment properly works with tarpaulin.
        #[cfg(not(tarpaulin_include))]
        tokio::spawn(async move {
            #[cfg(not(tarpaulin_include))]
            debug!("Background JWT key refresh task started");

            // Track operation timing for performance monitoring
            let start_time = std::time::Instant::now();

            use crate::oauth2::jwk::util_cache::cache_update_keys;

            // Fetch fresh keys from EVE's OAuth2 API
            #[cfg(not(tarpaulin_include))]
            debug!("Fetching fresh keys from JWK URL: {}", jwk_url);

            let result = fetch_jwt_keys(&reqwest_client, &jwk_url).await;
            let elapsed = start_time.elapsed();

            match result {
                // Fetch attempt was successful, JWT keys retrieved
                Ok(keys) => {
                    // Update the cache with the new keys
                    #[cfg(not(tarpaulin_include))]
                    debug!("Acquiring write lock for JWT keys cache update");

                    cache_update_keys(&jwt_key_cache, keys).await;

                    #[cfg(not(tarpaulin_include))]
                    info!(
                        "Background JWT key refresh task completed successfully in {}ms",
                        elapsed.as_millis()
                    );

                    // Clear any previous failure on success
                    let mut last_failure = last_refresh_failure.write().await;
                    *last_failure = None;

                    #[cfg(not(tarpaulin_include))]
                    debug!("Cleared previous JWT refresh failure timestamp");
                }
                // Fetch attempt for JWT keys failed
                Err(err) => {
                    #[cfg(not(tarpaulin_include))]
                    warn!(
                        "Background JWT key refresh failed after {}ms: {:?}",
                        elapsed.as_millis(),
                        err
                    );

                    // Record the failure time
                    let mut last_failure = last_refresh_failure.write().await;
                    *last_failure = Some(Instant::now());

                    #[cfg(not(tarpaulin_include))]
                    debug!("Recorded JWT refresh failure timestamp");
                }
            };

            // Release lock regardless of success
            jwk_refresh_lock_release_and_notify(
                &jwt_key_cache.refresh_lock,
                &jwt_key_cache.refresh_notifier,
            );
        });

        #[cfg(not(tarpaulin_include))]
        debug!("Background JWT key refresh task spawned successfully");

        true
    }
}
