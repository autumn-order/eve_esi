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

use log::debug;

use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
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
        debug!("Retrieving JWT keys");

        // Check if we have valid keys in the cache
        // If the cache is (80% to expiration) out of (1 hour), start a background task to refresh the keys
        if let Some(keys) = self.check_cache_and_trigger_background_refresh().await {
            return Ok(keys);
        }

        // If we got here, JWT key cache is missing or expired
        // Check if the keys are already being refreshed on another thread
        if !self.cache_lock_try_acquire() {
            // Wait for the key refresh to complete and then return the keys or an
            // error if the refresh times out (5 seconds)
            return self.wait_for_ongoing_refresh().await;
        }

        // We have the lock, so refresh the cache
        // Attempt up to (2 retries) with an exponential (100 ms) backoff
        self.refresh_jwt_keys_with_retry().await
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
        self.cache_update_keys(fresh_keys.clone()).await;

        Ok(fresh_keys)
    }

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
}
