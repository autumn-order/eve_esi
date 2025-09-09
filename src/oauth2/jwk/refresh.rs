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

use crate::error::{Error, OAuthError};
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
    ///   field used by the [`Client`](crate::Client). By default this is 5 seconds.
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
    pub(super) async fn wait_for_ongoing_refresh(&self) -> Result<EveJwtKeys, Error> {
        let esi_client = self.client;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;
        let config = &jwt_key_cache.config;

        let start_time = Instant::now();

        debug!("Waiting for another thread to refresh JWT keys");

        // Create a future that waits for the notification
        let notify_future = jwt_key_cache.refresh_notifier.notified();

        trace!("Created notification future for JWT key refresh wait");

        let refresh_timeout = config.refresh_timeout;
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

            debug!("{}", error_message);

            // Return error indicating function timed out waiting JWT key refresh
            return Err(Error::OAuthError(OAuthError::JwtKeyRefreshTimeout(
                error_message,
            )));
        }

        // Attempt to retrieve keys from cache
        if let Some((keys, timestamp)) = jwt_key_cache.get_keys().await {
            // Ensure keys are not expired
            let elapsed_seconds = timestamp.elapsed().as_secs();
            if elapsed_seconds < config.cache_ttl.as_secs() {
                let message = format!(
                    "Successfully retrieved JWT keys from cache after waiting {}ms for refresh",
                    elapsed.as_millis()
                );

                debug!("{}", message);

                // Return keys if successfully retrieved from cache & not expired
                return Ok(keys);
            }
        }

        // If the refresh request failed then no keys will be found in the cache
        let error_message = format!(
            "JWT key cache still empty of expired after waiting {}ms for refresh. Likely due to a failure to refresh the keys.",
            elapsed.as_millis()
        );

        debug!("{}", error_message);

        // Return an error indicating no keys were found in cache
        Err(Error::OAuthError(OAuthError::JwtKeyRefreshFailure(
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
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Check if we are still in cooldown due to fetch failure within 60 second cooldown period
        if check_refresh_cooldown(&jwt_key_cache).await.is_some() {
            debug!("Respecting refresh cooldown, delaying JWT key refresh");

            return false;
        }

        // Attempt to acquire a lock to perform the refresh
        if !jwt_key_cache.refresh_lock_try_acquire() {
            debug!("JWT key refresh already in progress");

            return false;
        }

        debug!("Triggering background JWT refresh task");

        // Clone the required components
        let client_ref = esi_client.inner.clone();

        tokio::spawn(async move {
            // Make no retries as the background refresh utilizes a 60 second cooldown between attempts instead.
            refresh_jwt_keys(&client_ref.reqwest_client, &client_ref.jwt_key_cache, 0).await
        });

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
///    field used by the [`Client`](crate::Client). By default this is 100ms.
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
/// - `max_retries` ([`u32`]): The amount of retries to make if the first attempt fails
///
/// # Returns
/// - `Ok(`[`EveJwtKeys`]`)` if keys were successfully fetched and cached
/// - `Err(`[`EsiError`]`)` if all request attempts failed
pub(super) async fn refresh_jwt_keys(
    reqwest_client: &reqwest::Client,
    jwt_key_cache: &JwtKeyCache,
    max_retries: u32,
) -> Result<EveJwtKeys, Error> {
    let config = &jwt_key_cache.config;

    // Track operation timing for performance monitoring
    let start_time = std::time::Instant::now();

    // Attempt inital JWT key refresh

    trace!("Fetching JWT keys from JWK URL: {}", &config.jwk_url);

    let mut result = fetch_and_update_cache(&reqwest_client, &jwt_key_cache).await;

    // Retry logic - attempt retries if the initial fetch failed
    let mut retry_attempts = 0;
    while result.is_err() && retry_attempts < max_retries {
        let backoff_duration = Duration::from_millis(
            // Calculate exponential backoff duration:
            // Initial backoff (100ms default) multiplied by 2^retry_attempts
            // This causes wait time to double with each retry attempt
            config.refresh_backoff.as_millis() as u64 * 2u64.pow(retry_attempts),
        );

        let message = format!(
            "JWT key fetch failed. Retrying ({}/{}) after {}ms",
            retry_attempts + 1,
            config.refresh_max_retries,
            backoff_duration.as_millis()
        );

        debug!("{}", message);

        // Wait before retrying
        tokio::time::sleep(backoff_duration).await;

        // Try to fetch again

        let message = format!(
            "Retry attempt # {}: fetching JWT keys after backoff",
            retry_attempts + 1
        );

        debug!("{}", message);

        result = fetch_and_update_cache(&reqwest_client, &jwt_key_cache).await;
        retry_attempts += 1;
    }

    // Always release the lock
    jwt_key_cache.refresh_lock_release_and_notify();

    // Return the result or error
    let elapsed = start_time.elapsed();
    match result {
        Ok(keys) => {
            let message = format!(
                "Successfully fetched and cached {} JWT keys (took {}ms)",
                keys.keys.len(),
                elapsed.as_millis()
            );

            info!("{}", message);

            // Clear any previous refresh failure on success
            jwt_key_cache.set_refresh_failure(None).await;

            debug!("Cleared previous JWT key refresh failure timestamp");

            // Return JWT keys
            Ok(keys)
        }
        Err(err) => {
            let message = format!(
                "JWT key refresh failed after {}ms: attempts={}, backoff_period={}ms, error={:?}",
                elapsed.as_millis(),
                retry_attempts,
                config.refresh_backoff.as_millis(),
                err
            );

            error!("{}", message);

            // Set the refresh failure time to prevent another refresh attempt within the
            // default 60 second cooldown period
            jwt_key_cache
                .set_refresh_failure(Some(std::time::Instant::now()))
                .await;

            debug!("Recorded JWT key refresh failure timestamp");

            // Return Error of type EsiError::ReqwestError
            Err(err.into())
        }
    }
}

#[cfg(test)]
mod wait_for_ongoing_refresh_tests {
    use crate::error::Error;
    use crate::oauth2::error::OAuthError;
    use crate::tests::setup;

    use super::super::tests::{create_mock_keys, get_jwk_internal_server_error_response};

    /// Validates retrieving keys from cache after waiting for refresh.
    ///
    /// Simulates waiting for another thread to finish refreshing JWT keys
    /// by acquiring a refresh lock and using a coroutine to simulate the
    /// refresh. Validates that the function properly returns expected
    /// keys after refresh finishes.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Configures a mock response returning an error 500 and expecting 0 requests
    /// - Acquire a lock on refreshing JWT keys
    /// - Spawn a coroutine to simulate another thread refreshing the keys
    ///
    /// # Assertions
    /// - Assert that refresh lock is in place
    /// - Assert no requests have been made to mock JWK endpoint
    /// - Assert result is ok
    #[tokio::test]
    async fn test_wait_for_refresh_success() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, mut mock_server) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Create mock response with error 500 and expecting 0 requests
        let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

        // Acquire a refresh lock
        let lock = jwt_key_cache.refresh_lock_try_acquire();

        // Assert that lock is in place
        assert_eq!(lock, true);

        // Create a channel to listen for when the coroutine starts
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Spawn a coroutine to perform the background refresh
        let keys = create_mock_keys();

        let keys_clone = keys.clone();
        let client_ref = esi_client.inner.clone();

        tokio::spawn(async move {
            // Signal that refresh is about to start
            let _ = tx.send(());

            // Simulate a network request delay
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            // Update keys
            client_ref.jwt_key_cache.update_keys(keys_clone).await;

            // Release lock & notify waiters
            client_ref.jwt_key_cache.refresh_lock_release_and_notify();
        });

        // Wait for coroutine to begin refresh
        rx.await.expect("Failed to receive ready signal");

        // Call method under test
        let result = esi_client.oauth2().jwk().wait_for_ongoing_refresh().await;

        // Assert mock server received 0 requests
        mock.assert();

        // Assert result is ok
        assert!(result.is_ok());
    }

    /// Validates error handling when the ongoing refresh fails and cache is empty
    ///
    /// Simulates waiting for another thread to finish refreshing JWT keys
    /// by acquiring a refresh lock and using a coroutine to simulate the
    /// refresh. Validates that the error is properly handled when the
    /// JWT key cache is not updated due to a refresh failure.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Configures a mock response returning an error 500 and expecting 0 requests
    /// - Don't set the cache with any keys which will be empty by default
    /// - Acquire a lock on refreshing JWT keys
    /// - Spawn a coroutine to simulate another thread failing to
    ///   refresh the keys
    ///
    /// # Assertions
    /// - Assert that refresh lock is in place
    /// - Assert no requests have been made to mock JWK endpoint
    /// - Assert result is error
    /// - Assert error is of type OAuthError::JwtKeyRefreshFailure
    #[tokio::test]
    async fn test_wait_for_refresh_failure_empty_cache() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, mut mock_server) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Create mock response with error 500 and expecting 0 requests
        let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

        // Acquire a refresh lock
        let lock = jwt_key_cache.refresh_lock_try_acquire();

        // Assert that lock is in place
        assert_eq!(lock, true);

        // Create a channel to listen for when the coroutine starts
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Spawn a coroutine to perform the background refresh
        let client_ref = esi_client.inner.clone();

        tokio::spawn(async move {
            // Signal that refresh is about to start
            let _ = tx.send(());

            // Simulate a network request delay
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            // Don't update the cache with keys to represent a failure

            // Release lock & notify waiters regardless of success
            client_ref.jwt_key_cache.refresh_lock_release_and_notify();
        });

        // Wait for coroutine to begin refresh
        rx.await.expect("Failed to receive ready signal");

        // Call method under test
        let result = esi_client.oauth2().jwk().wait_for_ongoing_refresh().await;

        // Assert mock server received 0 requests
        mock.assert();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type OAuthError::JwtKeyRefreshFailure
        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::JwtKeyRefreshFailure(_)))
        ));
    }

    /// Validates error handling when the ongoing refresh fails and cache is expired
    ///
    /// Ensures that keys are not returned despite cache is expired but not being empty
    ///
    /// Simulates waiting for another thread to finish refreshing JWT keys
    /// by acquiring a refresh lock and using a coroutine to simulate the
    /// refresh. Validates that the error is properly handled when the
    /// JWT key cache is not updated due to a refresh failure.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Configures a mock response returning an error 500 and expecting 0 requests
    /// - Populates cache with expired keys
    /// - Acquire a lock on refreshing JWT keys
    /// - Spawn a coroutine to simulate another thread failing to
    ///   refresh the keys
    ///
    /// # Assertions
    /// - Assert that refresh lock is in place
    /// - Assert no requests have been made to mock JWK endpoint
    /// - Assert result is error
    /// - Assert error is of type OAuthError::JwtKeyRefreshFailure
    #[tokio::test]
    async fn test_wait_for_refresh_failure_expired_cache() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, mut mock_server) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Create mock response with error 500 and expecting 0 requests
        let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

        // Set cache with expired keys
        {
            let expired_timestamp =
                std::time::Instant::now() - std::time::Duration::from_secs(3601);
            let mut cache = jwt_key_cache.cache.write().await;
            *cache = Some((create_mock_keys(), expired_timestamp));
        }

        // Acquire a refresh lock
        let lock = jwt_key_cache.refresh_lock_try_acquire();

        // Assert that lock is in place
        assert_eq!(lock, true);

        // Create a channel to listen for when the coroutine starts
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Spawn a coroutine to perform the background refresh
        let client_ref = esi_client.inner.clone();

        tokio::spawn(async move {
            // Signal that refresh is about to start
            let _ = tx.send(());

            // Simulate a network request delay
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            // Don't update the cache with keys to represent a failure

            // Release lock & notify waiters regardless of success
            client_ref.jwt_key_cache.refresh_lock_release_and_notify();
        });

        // Wait for coroutine to begin refresh
        rx.await.expect("Failed to receive ready signal");

        // Call method under test
        let result = esi_client.oauth2().jwk().wait_for_ongoing_refresh().await;

        // Assert mock server received 0 requests
        mock.assert();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type OAuthError::JwtKeyRefreshFailure
        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::JwtKeyRefreshFailure(_)))
        ));
    }

    /// Validates error handling when a timeout occurs waiting for refresh
    ///
    /// Simulates waiting for another thread to finish refreshing JWT keys
    /// before returning an error when the function times out waiting for
    /// the refresh that never finishes.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Configures a mock response returning an error 500 and expecting 0 requests
    /// - Acquire a lock on refreshing JWT keys
    /// - Cause a timeout by never notifying of a completed refresh
    ///
    /// # Assertions
    /// - Assert that refresh lock is in place
    /// - Assert no requests have been made to mock JWK endpoint
    /// - Assert result is error
    /// - Assert error is of type OAuthError::JwtKeyRefreshTimeout
    #[tokio::test]
    async fn test_wait_for_refresh_timeout() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, mut mock_server) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Create mock response with error 500 and expecting 0 requests
        let mock = get_jwk_internal_server_error_response(&mut mock_server, 0);

        // Acquire a refresh lock
        let lock = jwt_key_cache.refresh_lock_try_acquire();

        // Assert that lock is in place
        assert_eq!(lock, true);

        // Don't attempt any cache updates and don't release the lock which
        // should cause a timeout error.

        // Call method under test
        let result = esi_client.oauth2().jwk().wait_for_ongoing_refresh().await;

        // Assert mock server received 0 requests
        mock.assert();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type OAuthError::JwtKeyRefreshTimeout
        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::JwtKeyRefreshTimeout(_)))
        ));
    }
}

#[cfg(test)]
mod trigger_background_jwt_refresh_test {
    use std::time::Duration;

    use crate::tests::setup;

    /// Ensures the successful trigger of a background refresh
    ///
    /// Background refresh should occur because there is no current cooldown nor refresh lock in place.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    ///
    /// # Assertions
    /// - Assert background refresh has been triggered
    #[tokio::test]
    async fn test_background_refresh() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, _) = setup().await;

        // Trigger background refresh
        let result = esi_client
            .oauth2()
            .jwk()
            .trigger_background_jwt_refresh()
            .await;

        // Sleep to allow refresh to execute for code coverage
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Assert background refresh has been triggered
        assert_eq!(result, true);
    }

    /// Tests the background refresh if still within cooldown period
    ///
    /// A background refresh should not be triggered due to the last refresh failure
    /// being within the 60 second cooldown period.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Set last failure within cooldown period of last 60 seconds
    ///
    /// # Assertions
    /// - Assert background refresh was not triggered
    #[tokio::test]
    async fn test_background_refresh_cooldown() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, _) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Set last failure within cooldown period of last 60 seconds (failed 30 seconds ago)
        {
            let last_failure = std::time::Instant::now() - std::time::Duration::from_secs(30);

            let mut failure_time = jwt_key_cache.last_refresh_failure.write().await;
            *failure_time = Some(last_failure);
        }

        // Trigger background refresh
        let refresh_triggered = esi_client
            .oauth2()
            .jwk()
            .trigger_background_jwt_refresh()
            .await;

        // Assert background refresh was not triggered
        assert_eq!(refresh_triggered, false)
    }

    /// Tests the background refresh if refresh is already in progress by another thread
    ///
    /// Acquires a refresh lock which indicates another thread is performing a refresh,
    /// therefore a background refresh will not be triggered.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Acquire a refresh lock
    ///
    /// # Assertions
    /// - Assert refresh lock is in place
    /// - Assert background refresh was not triggered
    #[tokio::test]
    async fn test_background_refresh_already_in_progress() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, _) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Acquire a refresh lock
        let lock_acquired = jwt_key_cache.refresh_lock_try_acquire();

        // Assert refresh lock is in place
        assert_eq!(lock_acquired, true);

        // Trigger background refresh
        let refresh_triggered = esi_client
            .oauth2()
            .jwk()
            .trigger_background_jwt_refresh()
            .await;

        // Assert background refresh was not triggered
        assert_eq!(refresh_triggered, false);
    }
}

#[cfg(test)]
mod refresh_jwt_keys_tests {
    use crate::tests::setup;
    use crate::{error::Error, oauth2::jwk::refresh::refresh_jwt_keys};

    use super::super::tests::{get_jwk_internal_server_error_response, get_jwk_success_response};

    /// Validates successful refresh on first attempt
    ///
    /// Attempts to refresh & update JWT key cache from a mock server
    /// representing EVE Online OAuth2 API. Only 1 fetch attempt should be made as it will
    /// be a success on the first try.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Configures a mock success response with expected JWT keys
    ///
    /// # Assertions
    /// - Assert that only 1 fetch attempt was made to the server
    /// - Assert that the function returned the expected keys
    /// - Assert that the cache has been properly updated
    #[tokio::test]
    async fn test_refresh_keys_success() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, mut mock_server) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Create mock response with mock keys & expecting 1 request
        let mock = get_jwk_success_response(&mut mock_server, 1);

        // Call method under test
        let result = refresh_jwt_keys(
            &esi_client.inner.reqwest_client,
            &esi_client.inner.jwt_key_cache,
            esi_client.inner.jwt_key_cache.config.refresh_max_retries,
        )
        .await;

        // Assert we received only 1 expected request
        mock.assert();

        // Assert function returned expected keys
        assert!(result.is_ok());

        // Assert cache has been properly updated
        let cache = jwt_key_cache.cache.read().await;

        assert!(*&cache.is_some())
    }

    /// Validates error handling should all attempts fail
    ///
    /// Attempts to refresh & update JWT key cache from a mock server
    /// representing the EVE Online OAuth2 API. All attempts will fail due to the mock server returning
    /// error code 500 on each attempt. The function should retry
    /// for a total of 3 attempts before returning an error.
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Configures a mock response returning an error 500
    ///
    /// # Assertions
    /// - Assert we received only 3 expected requests
    /// - Assert result is an error
    /// - Assert error is of type Error::ReqwestError
    /// - Assert reqwest error is due to internal server error
    #[tokio::test]
    async fn test_refresh_keys_failure() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, mut mock_server) = setup().await;

        // Create mock response with error 500 and expecting 3 requests
        let mock = get_jwk_internal_server_error_response(&mut mock_server, 3);

        // Call method under test
        let result = refresh_jwt_keys(
            &esi_client.inner.reqwest_client,
            &esi_client.inner.jwt_key_cache,
            esi_client.inner.jwt_key_cache.config.refresh_max_retries,
        )
        .await;

        // Assert we received only 3 expected requests
        mock.assert();

        // Assert result is an error
        assert!(result.is_err());

        // Assert error is of type Error::ReqwestError
        assert!(matches!(result, Err(Error::ReqwestError(_))));

        // Assert reqwest error is due to internal server error
        assert!(
            matches!(result, Err(Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
        );
    }

    /// Validates successful refresh after 2 attempts
    ///
    /// Attempts to refresh & update JWT key cache from a mock server
    /// representing the EVE Online OAuth2 API. First attempt will fail due receiving status code 500,
    /// second attempt will succeed returning the expected keys
    ///
    /// # Test Setup
    /// - Create a basic Client & mock HTTP server
    /// - Configures an initial response returning an internal server error
    /// - Configures a second response that successfully returns the expected keys
    ///
    /// # Assertions
    /// - Assert that 1 fetch attempt was made for each response type, an
    ///   error 500 response and success 200 that returned expected keys
    /// - Assert that the function returned the expected keys
    /// - Assert that the cache has been properly updated
    #[tokio::test]
    async fn test_refresh_keys_retry() {
        // Setup a basic Client & mock HTTP server
        let (esi_client, mut mock_server) = setup().await;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;

        // Create an initial mock response with error 500 and expecting 1 request
        let mock_500 = get_jwk_internal_server_error_response(&mut mock_server, 1);

        // Create a 2nd mock response with mock keys & expecting 1 request
        let mock_200 = get_jwk_success_response(&mut mock_server, 1);

        // Call method under test
        let result = refresh_jwt_keys(
            &esi_client.inner.reqwest_client,
            &esi_client.inner.jwt_key_cache,
            esi_client.inner.jwt_key_cache.config.refresh_max_retries,
        )
        .await;

        // Assert we received only 1 expected request per response type
        mock_500.assert();
        mock_200.assert();

        // Assert function returned expected keys
        assert!(result.is_ok());

        // Assert cache has been properly updated
        let cache = jwt_key_cache.cache.read().await;

        assert!(*&cache.is_some())
    }
}
