//! Utility functions to retrieve EVE SSO JWT keys
//!
//! This module provides functions to fetch and cache JWT keys used for validating JWTs obtained from EVE's OAuth2 API.

use crate::error::{EsiError, OAuthError};
use crate::model::oauth2::EveJwtKeys;
use crate::EsiClient;

impl EsiClient {
    /// Retrieves the JWT keys used to validate JWTs obtained from EVE's OAuth2 API.
    ///
    /// # Returns
    /// A Result containing the JWT keys in successful, or an error if the fetch failed.
    ///
    /// # Errors
    /// - `EsiError::ReqwestError`: If the request to fetch JWT keys fails.
    pub(crate) async fn fetch_jwt_keys(&self) -> Result<EveJwtKeys, EsiError> {
        let jwt_keys = self
            .reqwest_client
            .get(self.eve_jwk_uri.to_string())
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
    ///
    /// # Errors
    /// - `OAuthError::CacheError`: This error case should never occur as we always update the cache after fetching fresh keys.
    pub async fn get_jwt_keys(&self) -> Result<EveJwtKeys, EsiError> {
        let needs_refresh = {
            let cache = self.jwt_keys_cache.lock().await;
            match &*cache {
                Some((_, timestamp)) => timestamp.elapsed().as_secs() >= self.jwt_keys_cache_ttl,
                None => true,
            }
        };

        if needs_refresh {
            let fresh_keys = self.fetch_jwt_keys().await?;
            let mut cache = self.jwt_keys_cache.lock().await;
            *cache = Some((fresh_keys.clone(), std::time::Instant::now()));
            return Ok(fresh_keys);
        }

        let cache = self.jwt_keys_cache.lock().await;
        match &*cache {
            Some((keys, _)) => Ok(keys.clone()),
            None => Err(EsiError::OAuthError(OAuthError::CacheError)),
        }
    }
}
