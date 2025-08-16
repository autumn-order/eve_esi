//! Utility functions to retrieve EVE SSO JWT keys
//!
//! This module provides methods to fetch and cache JWT keys used for validating JWTs
//! obtained from EVE's OAuth2 API.

use std::sync::atomic::Ordering;
use std::time::Instant;

use ::tokio::time::Duration;
use log::{debug, error, info};

use crate::constant::{
    DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF, DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
    DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_MAX_RETRIES, DEFAULT_JWK_REFRESH_TIMEOUT,
};
use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::error::OAuthError;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Fetches JWT keys from EVE's OAuth2 API regardless of the JWT key cache state.
    ///
    /// # Returns
    /// A Result containing the JWT keys in successful, or an error if the fetch failed.
    ///
    /// # Errors
    /// - `EsiError::ReqwestError`: If the request to fetch JWT keys fails.
    pub async fn fetch_jwt_keys(&self) -> Result<EveJwtKeys, EsiError> {
        let esi_client = self.client;
        let reqwest_client = &esi_client.reqwest_client;

        // Fetch fresh keys from EVE's OAuth2 API
        let jwt_keys = reqwest_client
            .get(self.client.jwk_url.to_string())
            .send()
            .await?
            .json()
            .await?;

        Ok(jwt_keys)
    }

    /// Retrieves JWT keys from EVE's OAuth2 API and updates the cache with the new keys
    ///
    /// Note: This method does not handle the jwt_key_refresh_in_progress flag itself.
    /// Use get_jwt_keys() instead for automatic handling of concurrent refresh attempts.
    ///
    /// # Returns
    /// - Result containing the JWT keys in successful, or an error if the fetch failed.
    ///
    /// # Errors
    /// - `EsiError::ReqwestError`: If the request to fetch JWT keys fails.
    pub async fn fetch_and_update_cache(&self) -> Result<EveJwtKeys, EsiError> {
        // Fetch fresh keys from EVE's OAuth2 API
        let fresh_keys = self.fetch_jwt_keys().await?;

        // Update the cache with the new keys
        {
            let mut cache = self.client.jwt_keys_cache.write().await;
            *cache = Some((fresh_keys.clone(), Instant::now()));
        }

        Ok(fresh_keys)
    }

    /// Gets JWT keys with caching support & background refreshing.
    ///
    /// This method returns JWT keys from the cache if available and not expired,
    /// otherwise it fetches fresh keys from EVE's OAuth2 API and updates the cache.
    ///
    /// If the cache is 80% to expiration, it will start a background task to refresh the keys
    /// proactively. This method prevents multiple concurrent refresh attempts by using an atomic
    /// flag. If a refresh is already in progress when this method is called, it will wait
    /// briefly and retry getting the keys from cache.
    ///
    /// # Returns
    /// - A Result containing the JWT keys in successful, or an error if the fetch failed.
    ///
    /// # Errors
    /// - Returns an error if the JWT key cache is empty and new keys could not be fetched.
    pub async fn get_jwt_keys(&self) -> Result<EveJwtKeys, EsiError> {
        let esi_client = self.client;
        debug!("Retrieving JWT keys");

        // Retrieve keys from cache
        let keys = {
            let cache = self.client.jwt_keys_cache.read().await;
            match &*cache {
                Some((keys, timestamp)) => Some((keys.clone(), timestamp.clone())),
                None => None,
            }
        }; // Lock is released here

        if let Some((keys, timestamp)) = keys {
            debug!("JWT keys found in cache");

            // Run a background refresh task if cache is at a certain % to expiration
            // TODO: make refresh threshold configurable
            // TODO: make backoff threshold configurable
            if timestamp.elapsed().as_secs()
                < (self.client.jwt_keys_cache_ttl
                    * DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT
                    / 100)
            {
                // Check if we should respect a backoff period due to previous failure
                let should_respect_backoff = {
                    match &*self.client.jwt_keys_last_refresh_failure.read().await {
                        Some(last_failure) => {
                            last_failure.elapsed().as_secs()
                                < DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF
                        }
                        None => false,
                    }
                };

                // Only trigger background refresh if not in backoff period and we can acquire the lock
                if !should_respect_backoff && self.try_acquire_refresh_lock() {
                    self.trigger_background_jwt_refresh().await;
                }
            }

            // Return keys if cache is not expired
            if timestamp.elapsed().as_secs() < self.client.jwt_keys_cache_ttl {
                debug!("Using cached JWT keys");
                return Ok(keys);
            } else {
                debug!("JWT keys cache expired");
            }
        } else {
            debug!("JWT keys cache miss");
        }

        // If we got here, JWT key cache is missing or expired
        if !self.try_acquire_refresh_lock() {
            debug!("Waiting for another thread to refresh JWT keys");

            // Create a future that waits for the notification
            let notify_future = self.client.jwt_key_refresh_notifier.notified();

            // Wait for the notification or a timeout (as fallback)
            tokio::select! {
                _ = notify_future => {
                    debug!("Received notification that JWT keys refresh is complete");
                }
                // TODO: configurable timeout
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
            let error_message = "Failed to fetch JWT keys after waiting for refresh".to_string();

            // Log the error at debug level
            debug!("{}", error_message);

            // Return appropriate error type
            return Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(
                error_message,
            )));
        }

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
        debug!("Releasing JWT key refresh lock");
        esi_client
            .jwt_key_refresh_in_progress
            .store(false, Ordering::Release);

        // Notify waiters regardless of success or failure
        // This is important - we must notify even if refresh failed
        self.client.jwt_key_refresh_notifier.notify_waiters();

        // Return the result or error
        match result {
            Ok(keys) => {
                debug!("Successfully fetched and cached fresh JWT keys");
                Ok(keys)
            }
            Err(err) => {
                let mut failure_time = self.client.jwt_keys_last_refresh_failure.write().await;
                *failure_time = Some(Instant::now());

                // Create a descriptive error message
                let error_message = format!(
                    "Failed to fetch JWT keys after {} attempts",
                    retry_attempts + 1
                );

                // Log the error at error level
                error!("{}: {}", error_message, err);

                // Return appropriate error type
                Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(
                    error_message,
                )))
            }
        }
    }
}
