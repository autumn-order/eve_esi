//! Utility functions to retrieve EVE SSO JWT keys
//!
//! This module provides functions to fetch and cache JWT keys used for validating JWTs obtained from EVE's OAuth2 API.

use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Retrieves the JWT keys used to validate JWTs obtained from EVE's OAuth2 API.
    ///
    /// # Returns
    /// A Result containing the JWT keys in successful, or an error if the fetch failed.
    ///
    /// # Errors
    /// - `EsiError::ReqwestError`: If the request to fetch JWT keys fails.
    pub async fn fetch_jwt_keys(&self) -> Result<EveJwtKeys, EsiError> {
        let jwt_keys = self
            .client
            .reqwest_client
            .get(self.client.jwk_url.to_string())
            .send()
            .await?
            .json()
            .await?;

        Ok(jwt_keys)
    }

    /// Gets JWT keys with caching support.
    ///
    /// This method returns JWT keys from the cache if available and not expired,
    /// otherwise it fetches fresh keys from EVE's OAuth2 API and updates the cache.
    ///
    /// # Returns
    /// A Result containing the JWT keys in successful, or an error if the fetch failed.
    pub async fn get_jwt_keys(&self) -> Result<EveJwtKeys, EsiError> {
        // First, check if we have valid cached keys
        {
            let cache = self.client.jwt_keys_cache.lock().await;
            if let Some((keys, timestamp)) = &*cache {
                if timestamp.elapsed().as_secs() < self.client.jwt_keys_cache_ttl {
                    // Cache is valid, return the keys
                    return Ok(keys.clone());
                }
            }
        } // Lock is released here

        // Fetch fresh keys from EVE's OAuth2 API
        let fresh_keys = self.fetch_jwt_keys().await?;

        // Update the cache with the new keys
        {
            let mut cache = self.client.jwt_keys_cache.lock().await;
            *cache = Some((fresh_keys.clone(), std::time::Instant::now()));
        }

        Ok(fresh_keys)
    }
}
