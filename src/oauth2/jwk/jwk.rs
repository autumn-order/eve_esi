//! Utility functions to retrieve EVE SSO JWT keys
//!
//! This module provides methods to fetch and cache JWT keys used for validating JWTs
//! obtained from EVE's OAuth2 API.

use std::time::Instant;

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
        // LOG: debug log for retrieving keys

        // Retrieve keys from cache
        let keys = {
            let cache = self.client.jwt_keys_cache.read().await;
            match &*cache {
                Some((keys, timestamp)) => Some((keys.clone(), timestamp.clone())),
                None => None,
            }
        }; // Lock is released here

        if let Some((keys, timestamp)) = keys {
            // LOG: Debug log that keys found in cache

            // Run a background refresh task if cache is 80% to expiration
            // TODO: make refresh threshold configurable
            if timestamp.elapsed().as_secs() < (self.client.jwt_keys_cache_ttl * 8 / 10) {
                if self.try_acquire_refresh_lock() {
                    self.trigger_background_jwt_refresh().await;
                }
            }

            // Return keys if cache is not expired
            if timestamp.elapsed().as_secs() < self.client.jwt_keys_cache_ttl {
                // LOG: Debug log using cached keys
                return Ok(keys);
            }

            // ELSE
            // LOG: Debug log cache expiration
        }
        // ELSE
        // LOG: Debug log cache miss

        // If we got here, JWT key cache is missing or expired
        // TODO: Utilize a notification mechanism so that the threads wait
        // precisely rather than approximately as long as it takes to refresh the keys
        // Try to get the refresh lock
        // TODO: Retry with exponential backoff
        // TODO: configurable number of retry attempts
        if !self.try_acquire_refresh_lock() {
            // LOG: Debug log waiting for another thread

            // Another thread is refreshing, wait briefly
            // TODO: make adjustable rather than hard-coded
            // TODO: Make the default sleep duration a static in constant.rs
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // Try cache again after waiting
            if let Some(keys) = self.get_keys_from_cache().await {
                // LOG: Debug log successful retrieval after waiting
                return Ok(keys);
            }
            // LOG: Warn log failed key retrieval after waiting
            return Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError));
        }

        // LOG: info log before fetching fresh keys

        // We have the lock, so refresh the cache
        let fresh_keys = self.fetch_and_update_cache().await?;
        // MATCH for LOG
        // OK
        // LOG: info log successfuly fetched and cached fresh keys
        // ERR
        // LOG: error log failed to fetch fresh JWT keys

        // Always release the lock
        // LOG: debug log lock release
        esi_client
            .jwt_key_refresh_in_progress
            .store(false, std::sync::atomic::Ordering::Release);

        Ok(fresh_keys)
    }
}
