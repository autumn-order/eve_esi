use std::sync::atomic::Ordering;
use std::time::Instant;

use ::tokio::time::Duration;
use log::{debug, error, info};

use crate::constant::{DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_MAX_RETRIES};
use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Helper function to trigger a background JWT refresh task.
    pub async fn trigger_background_jwt_refresh(&self) {
        debug!("Triggering background JWT refresh task");

        let esi_client = self.client;

        // Clone the required components
        let reqwest_client = esi_client.reqwest_client.clone();
        let jwt_keys_cache = esi_client.jwt_keys_cache.clone();
        let jwk_url = esi_client.jwk_url.clone();
        let refresh_in_progress = esi_client.jwt_key_refresh_in_progress.clone();
        let jwt_key_refresh_notifier = esi_client.jwt_key_refresh_notifier.clone();
        let jwt_keys_last_refresh_failure = esi_client.jwt_keys_last_refresh_failure.clone();

        tokio::spawn(async move {
            debug!("Background JWT key refresh task started");

            let result = async {
                debug!("Fetching fresh keys from JWK URL: {}", jwk_url);

                // Fetch fresh keys from EVE's OAuth2 API
                let fresh_keys = reqwest_client
                    .get(jwk_url.to_string())
                    .send()
                    .await?
                    .json::<EveJwtKeys>()
                    .await?;

                // Update the cache with the new keys
                debug!("Updating JWT keys cache");
                {
                    let mut cache = jwt_keys_cache.write().await;
                    *cache = Some((fresh_keys, Instant::now()));
                }

                debug!("JWT keys cache updated");
                Ok::<_, EsiError>(())
            }
            .await;

            // Always release the lock
            debug!("Releasing JWT key refresh lock");
            refresh_in_progress.store(false, Ordering::Release);

            // Notify waiting threads that the cache has been updated
            jwt_key_refresh_notifier.notify_waiters();

            if let Err(err) = result {
                error!("Background JWT key refresh failed: {:?}", err);

                // Record the failure time
                let mut last_failure = jwt_keys_last_refresh_failure.write().await;
                *last_failure = Some(Instant::now());
            } else {
                debug!("Background JWT key refresh task successful");

                // Clear any previous failure on success
                let mut last_failure = jwt_keys_last_refresh_failure.write().await;
                *last_failure = None;
            }
        });

        debug!("Background JWT key refresh task spawned");
    }

    /// Refreshes JWT keys with retry logic
    /// This function assumes the refresh lock is already acquired
    pub async fn refresh_jwt_keys_with_retry(&self) -> Result<EveJwtKeys, EsiError> {
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
        self.release_refresh_lock_and_notify();

        // Return the result or error
        match result {
            Ok(keys) => {
                debug!("Successfully fetched and cached fresh JWT keys");
                // Clear any previous failure on success
                let mut last_failure = self.client.jwt_keys_last_refresh_failure.write().await;
                *last_failure = None;
                Ok(keys)
            }
            Err(_) => {
                // Record the failure with retry count + 1 (to account for initial attempt)
                let error = self
                    .record_refresh_failure(DEFAULT_JWK_REFRESH_MAX_RETRIES + 1)
                    .await;
                Err(error)
            }
        }
    }
}
