use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Utility to try to acquire the refresh lock for updating JWT keys
    pub fn try_acquire_refresh_lock(&self) -> bool {
        !self
            .client
            .jwt_key_refresh_in_progress
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
    }

    /// Utility to check the JWT key cache and get keys
    pub async fn get_keys_from_cache(&self) -> Option<EveJwtKeys> {
        let cache = self.client.jwt_keys_cache.read().await;
        match &*cache {
            Some((keys, _)) => Some(keys.clone()),
            None => None,
        }
    }
}
