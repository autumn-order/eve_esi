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

    /// Gets JWT keys with caching support.
    ///
    /// This method returns JWT keys from the cache if available and not expired,
    /// otherwise it fetches fresh keys from EVE's OAuth2 API and updates the cache.
    ///
    /// This method prevents multiple concurrent refresh attempts by using an atomic flag.
    /// If a refresh is already in progress when this method is called, it will wait
    /// briefly and retry getting the keys from cache.
    ///
    /// # Errors
    /// - Returns an error if the JWT key cache is empty and new keys could not be fetched.
    ///
    /// # Returns
    /// - A Result containing the JWT keys in successful, or an error if the fetch failed.
    pub async fn get_jwt_keys(&self) -> Result<EveJwtKeys, EsiError> {
        // TODO: Proactively refresh keys before they expire
        let esi_client = self.client;

        // First, check if we have valid cached keys
        {
            let cache = self.client.jwt_keys_cache.read().await;
            if let Some((keys, timestamp)) = &*cache {
                if timestamp.elapsed().as_secs() < self.client.jwt_keys_cache_ttl {
                    // Cache is valid, return the keys
                    return Ok(keys.clone());
                }
            }
        } // Lock is released here

        // Try to set the refresh flag if not already set
        let was_already_refreshing = esi_client
            .jwt_key_refresh_in_progress
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err();

        // TODO: Retry with exponential backoff
        // TODO: configurable number of retry attempts
        if was_already_refreshing {
            // Another thread is already refreshing, wait briefly and check cache again
            // TODO: make adjustable rather than hard-coded
            // TODO: Make the default sleep duration a static in constant.rs
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // Retry getting from cache after waiting
            let cache = esi_client.jwt_keys_cache.read().await;
            if let Some((keys, _)) = &*cache {
                return Ok(keys.clone());
            }

            // If still no keys in cache after waiting, return an error
            return Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError));
        }

        // We successfully set the flag, so we're responsible for refreshing
        let result = self.fetch_and_update_cache().await;

        // Reset the flag regardless of whether the fetch succeeded or failed
        esi_client
            .jwt_key_refresh_in_progress
            .store(false, std::sync::atomic::Ordering::Release);

        result
    }
}
