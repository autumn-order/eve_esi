//! Core JWT Key API Functions
//!
//! This module provides the primary public interfaces for JWT key operations,
//! including fetching, caching, and retrieving keys for JWT validation.
//!
//! The main functions in this module serve as the entry points for JWT key
//! operations in the EVE ESI OAuth2 flow. They orchestrate the interaction
//! between cache management, refresh tasks, and external API calls.
//!
//! See the [module-level documentation](super) for a more detailed overview and usage.

use log::{debug, error};
use std::time::Instant;

use crate::error::{Error, OAuthError};
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::jwk::cache::JwtKeyCache;
use crate::oauth2::jwk::refresh::refresh_jwt_keys;
use crate::oauth2::jwk::util::{
    check_refresh_cooldown, is_cache_approaching_expiry, is_cache_expired,
};
use crate::oauth2::OAuth2Api;
use crate::Client;

/// Provides access to JWK endpoints & caching for EVE Online's OAuth2 endpoints
///
/// The [`JwkApi`] acts as an interface for retrieving JWT keys, caching, and refreshing them
/// when the keys are expired or nearing expiration.
///
/// It requires an [`Client`] which is used for making HTTP requests and it provides the
/// JWT key cache used by the eve_esi crate's OAuth2 functionality for token validation.
///
/// See the [module-level documentation](super) for an overview and usage example.
pub struct JwkApi<'a> {
    pub(super) client: &'a Client,
}

impl OAuth2Api<'_> {
    /// Access to EVE Online's OAuth2 JWK endpoints used to validate tokens
    ///
    /// Additionally provides access to methods that implement caching and refreshing
    /// when the JWT keys are expired or nearing expiration.
    ///
    /// Return an API client for interacting with the JWK endpoints.
    pub fn jwk(&self) -> self::JwkApi<'_> {
        self::JwkApi::new(&self.client)
    }
}

impl<'a> JwkApi<'a> {
    /// Creates a new instance of [`JwkApi`]
    ///
    /// # Arguments
    /// - `client` (&'a [`Client`]) used for making HTTP requests to EVE Online API endpoints
    ///   and providing the JWT key cache.
    ///
    /// # Returns
    /// - `Self`: A new instance of [`JwkApi`].
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Gets JWT keys with caching support & background refreshing.
    ///
    /// This method returns JWT keys from the cache if available and not expired,
    /// otherwise it fetches fresh keys from EVE's OAuth2 API and updates the cache.
    ///
    /// If the cache is 80% to expiration by default, it will start a background task to refresh the
    /// keys proactively. This method prevents multiple concurrent refresh attempts by using an atomic
    /// flag. If a refresh is already in progress when this method is called, it will wait
    /// for the refresh to complete and then try getting the keys from cache.
    ///
    /// # Implementation Details
    /// - Uses a read lock on the cache to check current state without blocking other readers
    /// - Implements the "refresh ahead" pattern to update cache before key expiry
    /// - If keys are expired, ensures no refresh failure occurred within the last 60 seconds
    ///   before attempting to acquire a lock to refresh the keys.
    /// - If a refresh lock is already in place, waits for notification of a completed
    ///   refresh and then returns the keys from the cache if successful.
    ///
    /// # Thread Safety
    /// This method is thread-safe and can be called concurrently by multiple threads.
    /// It uses appropriate locking to ensure consistency when reading the cache while
    /// preventing multiple simultaneous refresh operations.
    ///
    /// # Returns
    /// - [`EveJwtKeys`]: A Result containing the JWT keys if successful
    ///
    /// # Errors
    /// - [`Error`]: Returns an error if the JWT key cache is empty and new keys could not be fetched.
    pub async fn get_jwt_keys(&self) -> Result<EveJwtKeys, Error> {
        let esi_client = self.client;
        let jwt_key_cache = &esi_client.inner.jwt_key_cache;
        let config = &jwt_key_cache.config;

        // Check if we have valid keys in the cache
        #[cfg(not(tarpaulin_include))]
        debug!("Checking JWT keys cache state");

        if let Some((keys, timestamp)) = jwt_key_cache.get_keys().await {
            let elapsed_seconds = timestamp.elapsed().as_secs();

            // If the cache is not expired return the keys
            if !is_cache_expired(&jwt_key_cache, elapsed_seconds) {
                // If background refresh is enabled & the cache is approaching expiry, trigger a background refresh
                if jwt_key_cache.config.background_refresh_enabled
                    && is_cache_approaching_expiry(&jwt_key_cache, elapsed_seconds)
                {
                    #[cfg(not(tarpaulin_include))]
                    debug!("JWT keys approaching expiry (age: {}s)", elapsed_seconds);

                    // If the cache is 80% to expiration out of 1 hour, start a refresh
                    // This function will also check:
                    // - If a refresh failure occurred recently within cooldown period of 60 seconds
                    // - If a refresh is already progress, if so it won't spawn another refresh task
                    let _ = self.trigger_background_jwt_refresh().await;
                }

                #[cfg(not(tarpaulin_include))]
                debug!(
                    "JWT keys still valid, using keys from cache (age: {}s)",
                    elapsed_seconds
                );

                return Ok(keys);
            } else {
                #[cfg(not(tarpaulin_include))]
                debug!(
                    "JWT key cache expired (age: {}s)",
                    timestamp.elapsed().as_secs()
                );
            }
        } else {
            #[cfg(not(tarpaulin_include))]
            debug!("JWT key cache is currently empty");
        };

        // Return error if JWT key refresh is still within default 60 second cooldown period
        //
        // If a recent attempt to refresh keys was made and all retries failed, a 60
        // second cooldown period will be active until the next set of attempts.
        let cooldown = check_refresh_cooldown(&jwt_key_cache).await;
        if let Some(cooldown_remaining) = cooldown {
            let error_message = format!(
                "JWT key refresh cooldown still active due to recent refresh failure during last {} seconds. Cooldown remaining: {} seconds.",
                &config.refresh_cooldown.as_secs(), cooldown_remaining
            );

            #[cfg(not(tarpaulin_include))]
            error!("{}", error_message);

            return Err(Error::OAuthError(OAuthError::JwtKeyRefreshCooldown(
                error_message,
            )));
        }

        // If we got here, JWT key cache is missing or expired
        // Check if the keys are already being refreshed on another thread
        if !jwt_key_cache.refresh_lock_try_acquire() {
            // Wait for the key refresh to complete and then return the keys or an
            // error if the refresh times out (5 seconds)
            return self.wait_for_ongoing_refresh().await;
        }

        // We have the lock, so refresh the cache
        // Attempt up to (2 retries) with an exponential (100 ms) backoff
        refresh_jwt_keys(
            &esi_client.inner.reqwest_client,
            &jwt_key_cache,
            jwt_key_cache.config.refresh_max_retries,
        )
        .await
    }

    /// Retrieves JWT keys from EVE's OAuth2 API and updates the cache with the new keys
    ///
    /// This method fetches JWT keys from EVE's OAuth2 API and immediately updates the
    /// cache & returns the keys if successful.
    ///
    /// This function does not implement measures to prevent concurrent JWT key fetch
    /// attempts, you should use [`Self::get_jwt_keys`] if you do not wish to implement
    /// these mechanics yourself.
    ///
    /// # Returns
    /// - [`EveJwtKeys`]: Struct representing JWT keys returned from the EVE OAuth2 JWK endpoint.
    ///
    /// # Errors
    /// - [`Error::ReqwestError`]: If the request to fetch JWT keys fails.
    pub async fn fetch_and_update_cache(&self) -> Result<EveJwtKeys, Error> {
        let esi_client = self.client;

        fetch_and_update_cache(
            &esi_client.inner.reqwest_client,
            &esi_client.inner.jwt_key_cache,
        )
        .await
    }

    /// Fetches JWT keys from EVE's OAuth2 API
    ///
    /// This function does not implement measures to prevent concurrent JWT key fetch
    /// attempts, you should use [`Self::get_jwt_keys`] if you do not wish to implement
    /// these mechanics yourself.
    ///
    /// # Returns
    /// - [`EveJwtKeys`]: Struct representing JWT keys returned from the EVE OAuth2 JWK endpoint.
    ///
    /// # Errors
    /// - [`Error::ReqwestError`]: If the request to fetch JWT keys fails.
    pub async fn fetch_jwt_keys(&self) -> Result<EveJwtKeys, Error> {
        let esi_client = self.client;

        fetch_jwt_keys(
            &esi_client.inner.reqwest_client,
            &esi_client.inner.jwt_key_cache.config.jwk_url,
        )
        .await
    }
}

/// Utility function for fetching jwt key
///
/// Fetches JWT keys from EVE's OAuth2 API and returns the keys if
/// successful or a reqwest error if not.
///
/// See [`crate::oauth2::OAuth2Api::fetch_jwt_keys`] for public facing
/// method for fetching JWT keys.
///
/// # Arguments
/// - `reqwest_client` (&[`reqwest::Client`]): HTTP client used to make requests
/// - `jwk_url` (&[`str`]): String representing the JWK endpoint for EVE's OAuth2 API
///
/// # Returns
/// - [`EveJwtKeys`]: a struct containing the JWT keys if successful
///
/// # Errors
/// - [`Error::ReqwestError`]: If the request to fetch JWT keys fails.
pub(super) async fn fetch_jwt_keys(
    reqwest_client: &reqwest::Client,
    jwk_url: &str,
) -> Result<EveJwtKeys, Error> {
    #[cfg(not(tarpaulin_include))]
    debug!("Fetching JWT keys from EVE OAuth2 API: {}", jwk_url);

    let start_time = Instant::now();

    // Fetch fresh keys from EVE's OAuth2 API
    let response = match reqwest_client.get(jwk_url.to_string()).send().await {
        Ok(resp) => {
            #[cfg(not(tarpaulin_include))]
            debug!(
                "Received response from JWT keys endpoint, status: {}",
                resp.status()
            );

            // If server response status code is an error, return an error
            if let Err(err) = resp.error_for_status_ref() {
                return Err(err.into());
            }

            resp
        }
        // Typically connection/request related errors
        Err(e) => {
            let elapsed = start_time.elapsed();

            #[cfg(not(tarpaulin_include))]
            error!(
                "Failed to connect to JWT keys endpoint after {}ms: {:?}",
                elapsed.as_millis(),
                e
            );

            return Err(e.into());
        }
    };

    // Convert response body into EveJwtKeys struct
    let jwt_keys = match response.json::<EveJwtKeys>().await {
        Ok(keys) => {
            let elapsed = start_time.elapsed();

            #[cfg(not(tarpaulin_include))]
            debug!(
                "Successfully parsed JWT keys response with {} keys (took {}ms)",
                keys.keys.len(),
                elapsed.as_millis()
            );

            keys
        }
        // Error related to parsing the body to the EveJwtKeys struct
        Err(e) => {
            let elapsed = start_time.elapsed();

            #[cfg(not(tarpaulin_include))]
            error!(
                "Failed to parse JWT keys response after {}ms: {:?}",
                elapsed.as_millis(),
                e
            );

            return Err(e.into());
        }
    };

    Ok(jwt_keys)
}

/// Utility function for fetching jwt keys & updating cache
///
/// Fetches JWT keys from EVE's OAuth2 API and updates the JWT key cache
/// if successful, returns the resulting keys or error.
///
/// See [`crate::oauth2::OAuth2Api::fetch_and_update_cache`] for public facing
/// method for fetching JWT keys.
///
/// # Arguments
/// - `reqwest_client` (&[`reqwest::Client`]): HTTP client used to make requests
/// - `jwk_url` (&[`str`]): String representing the JWK endpoint for EVE's OAuth2 API
/// - `jwt_key_cache` (&[`JwtKeyCache`]): JWT Key cache struct with methods to get & update keys
///   and coordinate concurrent refresh attempts.
///
/// # Returns
/// - [`EveJwtKeys`]: a struct containing the JWT keys if successful
///
/// # Errors
/// - [`Error::ReqwestError`]: If the request to fetch JWT keys fails.
pub(super) async fn fetch_and_update_cache(
    reqwest_client: &reqwest::Client,
    jwt_key_cache: &JwtKeyCache,
) -> Result<EveJwtKeys, Error> {
    #[cfg(not(tarpaulin_include))]
    debug!("Fetching fresh JWT keys and updating cache");

    let start_time = Instant::now();

    // Fetch fresh keys from EVE's OAuth2 API
    let fetch_result = fetch_jwt_keys(reqwest_client, &jwt_key_cache.config.jwk_url).await;

    match fetch_result {
        Ok(fresh_keys) => {
            #[cfg(not(tarpaulin_include))]
            debug!(
                "Successfully fetched {} JWT keys, updating cache",
                fresh_keys.keys.len()
            );

            // Update the cache with the new keys
            jwt_key_cache.update_keys(fresh_keys.clone()).await;

            let elapsed = start_time.elapsed();

            #[cfg(not(tarpaulin_include))]
            debug!(
                "JWT keys cache updated successfully (took {}ms)",
                elapsed.as_millis()
            );

            Ok(fresh_keys)
        }
        Err(e) => {
            let elapsed = start_time.elapsed();

            #[cfg(not(tarpaulin_include))]
            error!(
                "Failed to fetch JWT keys after {}ms: {:?}",
                elapsed.as_millis(),
                e
            );

            Err(e)
        }
    }
}
