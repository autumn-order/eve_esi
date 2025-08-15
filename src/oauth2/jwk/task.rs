use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

use std::time::Instant;

impl<'a> OAuth2Api<'a> {
    /// Helper function to trigger a background JWT refresh task.
    pub async fn trigger_background_jwt_refresh(&self) {
        // LOG: debug log trigger background JWT refresh task

        let esi_client = self.client;
        // Clone the required components
        let reqwest_client = esi_client.reqwest_client.clone();
        let jwt_keys_cache = esi_client.jwt_keys_cache.clone();
        let jwk_url = esi_client.jwk_url.clone();
        let refresh_in_progress = esi_client.jwt_key_refresh_in_progress.clone();

        tokio::spawn(async move {
            // LOG: debug log background JWT key refresh task started

            let result = async {
                // LOG: debug log fetching fresh keys from JWK URL

                // Fetch fresh keys from EVE's OAuth2 API
                let fresh_keys = reqwest_client
                    .get(jwk_url.to_string())
                    .send()
                    .await?
                    .json::<EveJwtKeys>()
                    .await?;

                // Update the cache with the new keys
                // LOG: debug log updating JWT keys cache
                {
                    let mut cache = jwt_keys_cache.write().await;
                    *cache = Some((fresh_keys, Instant::now()));
                }
                // LOG: Info log JWT keys cache updated
                Ok::<_, EsiError>(())
            }
            .await;

            // Always reset the refresh flag
            // LOG: debug log reset JWT key refresh flag
            refresh_in_progress.store(false, std::sync::atomic::Ordering::Release);

            if let Err(err) = result {
                // LOG: error log background JWT key refresh failed
                eprintln!("Background JWT key refresh failed: {:?}", err);
                // LOG: debug log background JWT key refresh task successful
            }
        });

        // LOG: debug log JWT key refresh task spawned
    }
}
