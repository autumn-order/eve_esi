use std::sync::atomic::Ordering;
use std::time::Instant;

use log::{debug, error};

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
            } else {
                debug!("Background JWT key refresh task successful");
            }
        });

        debug!("Background JWT key refresh task spawned");
    }
}
