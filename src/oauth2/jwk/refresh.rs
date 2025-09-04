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
use log::{debug, error, info, trace};

use crate::error::{EsiError, OAuthError};
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::jwk::cache::JwtKeyCache;

use super::jwk::{fetch_and_update_cache, JwkApi};
use super::util::check_refresh_cooldown;

impl<'a> JwkApi<'a> {
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

        let start_time = Instant::now();

        #[cfg(not(tarpaulin_include))]
        debug!("Waiting for another thread to refresh JWT keys");

        // Create a future that waits for the notification
        let notify_future = jwt_key_cache.refresh_notifier.notified();

        #[cfg(not(tarpaulin_include))]
        trace!("Created notification future for JWT key refresh wait");

        let refresh_timeout = Duration::from_secs(jwt_key_cache.refresh_timeout);
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
            return Err(EsiError::OAuthError(OAuthError::JwtKeyRefreshTimeout(
                error_message,
            )));
        }

        // Attempt to retrieve keys from cache
        if let Some((keys, timestamp)) = jwt_key_cache.get_keys().await {
            // Ensure keys are not expired
            let elapsed_seconds = timestamp.elapsed().as_secs();
            if elapsed_seconds < jwt_key_cache.cache_ttl {
                #[cfg(not(tarpaulin_include))]
                debug!(
                    "Successfully retrieved JWT keys from cache after waiting {}ms for refresh",
                    elapsed.as_millis()
                );

                // Return keys if successfully retrieved from cache & not expired
                return Ok(keys);
            }
        }

        // If the refresh request failed then no keys will be found in the cache
        let error_message = format!(
            "JWT key cache still empty of expired after waiting {}ms for refresh. Likely due to a failure to refresh the keys.",
            elapsed.as_millis()
        );

        #[cfg(not(tarpaulin_include))]
        debug!("{}", error_message);

        // Return an error indicating no keys were found in cache
        Err(EsiError::OAuthError(OAuthError::JwtKeyRefreshFailure(
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

        // Check if we are still in cooldown due to fetch failure within 60 second cooldown period
        if check_refresh_cooldown(&jwt_key_cache).await.is_some() {
            #[cfg(not(tarpaulin_include))]
            debug!("Respecting refresh cooldown, delaying JWT key refresh");

            return false;
        }

        // Attempt to acquire a lock to perform the refresh
        if !jwt_key_cache.refresh_lock_try_acquire() {
            #[cfg(not(tarpaulin_include))]
            debug!("JWT key refresh already in progress");

            return false;
        }

        #[cfg(not(tarpaulin_include))]
        debug!("Triggering background JWT refresh task");

        // Clone the required components
        let reqwest_client = esi_client.reqwest_client.clone();
        let jwt_key_cache = esi_client.jwt_key_cache.clone();

        tokio::spawn(async move {
            // Make no retries as the background refresh utilizes a 60 second cooldown between attempts instead.
            refresh_jwt_keys(&reqwest_client, &jwt_key_cache, 0).await
        });

        #[cfg(not(tarpaulin_include))]
        debug!("Background JWT key refresh task started");

        true
    }
}

/// Refreshes JWT keys with retry logic
///
/// This method implements a blocking refresh operation with exponential backoff retry:
/// 1. Attempts to fetch JWT keys from the EVE OAuth2 API & update the cache
/// 2. If initial attempt fails, retries with exponential backoff delay defined by the
///    [`OAuthConfig::jwk_refresh_backoff`](crate::oauth2::OAuth2Config::jwk_refresh_backoff)
///    field used by the [`EsiClient`](crate::EsiClient). By default this is 100ms.
/// 3. Continues retrying until success or maximum retry count provided is reached.
/// 4. Releases the refresh lock and notifies waiting threads upon completion regardless of success.
/// 5. Records refresh failures for a cooldown between a set of refresh attempts
///
/// # Implementation Details
/// - Uses exponential backoff to gracefully handle temporary service issues
/// - Assumes the refresh lock is already acquired before being called
/// - Always releases the lock upon completion (success or failure)
/// - Updates the cache on successful refresh
/// - Records failure information for future cooldown decisions
///
/// # Thread Safety
/// This method is thread-safe and designed to be called only when the refresh lock
/// has been acquired. It properly releases the lock when done, ensuring other
/// threads can proceed with their operations.
///
/// # Arguments
/// - `reqwest_client` (&[`reqwest::Client`]): Client used for making HTTP requests
/// - `jwt_key_cache` (&[`JwtKeyCache`]): Cache providing methods to get, update, and coordinate JWT key refreshes
/// - `jwk_url` (&[`str`]): URL endpoint to retrieve the JWT keys from
/// - `backoff` ([`u64`]): The exponential backoff in ms between request attempts
/// - `max_retries` ([`u64`]): The amount of retries to make if the first attempt fails
///
/// # Returns
/// - `Ok(`[`EveJwtKeys`]`)` if keys were successfully fetched and cached
/// - `Err(`[`EsiError`]`)` if all request attempts failed
pub(super) async fn refresh_jwt_keys(
    reqwest_client: &reqwest::Client,
    jwt_key_cache: &JwtKeyCache,
    max_retries: u64,
) -> Result<EveJwtKeys, EsiError> {
    // Track operation timing for performance monitoring
    let start_time = std::time::Instant::now();

    // Attempt inital JWT key refresh
    #[cfg(not(tarpaulin_include))]
    debug!("Fetching JWT keys from JWK URL: {}", &jwt_key_cache.jwk_url);

    let mut result = fetch_and_update_cache(&reqwest_client, &jwt_key_cache).await;

    // Retry logic - attempt retries if the initial fetch failed
    let mut retry_attempts = 0;
    while result.is_err() && retry_attempts < max_retries {
        let backoff_duration = Duration::from_millis(
            // Calculate exponential backoff duration:
            // Initial backoff (100ms default) multiplied by 2^retry_attempts
            // This causes wait time to double with each retry attempt
            jwt_key_cache.refresh_backoff * 2u64.pow(retry_attempts as u32),
        );

        #[cfg(not(tarpaulin_include))]
        debug!(
            "JWT key fetch failed. Retrying ({}/{}) after {}ms",
            retry_attempts + 1,
            jwt_key_cache.refresh_max_retries,
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

        result = fetch_and_update_cache(&reqwest_client, &jwt_key_cache).await;
        retry_attempts += 1;
    }

    // Always release the lock
    jwt_key_cache.refresh_lock_release_and_notify();

    // Return the result or error
    let elapsed = start_time.elapsed();
    match result {
        Ok(keys) => {
            #[cfg(not(tarpaulin_include))]
            info!(
                "Successfully fetched and cached {} JWT keys (took {}ms)",
                keys.keys.len(),
                elapsed.as_millis()
            );

            // Clear any previous refresh failure on success
            jwt_key_cache.set_refresh_failure(None).await;

            #[cfg(not(tarpaulin_include))]
            debug!("Cleared previous JWT key refresh failure timestamp");

            // Return JWT keys
            Ok(keys)
        }
        Err(err) => {
            #[cfg(not(tarpaulin_include))]
            error!(
                "JWT key refresh failed after {}ms: attempts={}, backoff_period={}ms, error={:?}",
                elapsed.as_millis(),
                retry_attempts,
                jwt_key_cache.refresh_backoff,
                err
            );

            // Set the refresh failure time to prevent another refresh attempt within the
            // default 60 second cooldown period
            jwt_key_cache
                .set_refresh_failure(Some(std::time::Instant::now()))
                .await;

            #[cfg(not(tarpaulin_include))]
            debug!("Recorded JWT key refresh failure timestamp");

            // Return Error of type EsiError::ReqwestError
            Err(err.into())
        }
    }
}
