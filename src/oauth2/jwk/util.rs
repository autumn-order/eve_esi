use ::tokio::time::Duration;
use log::{debug, error};

use crate::constant::{
    DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF, DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
    DEFAULT_JWK_REFRESH_TIMEOUT,
};
use crate::error::EsiError;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::error::OAuthError;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Utility that waits for an ongoing JWT key cache refresh operation to complete and returns the result
    pub async fn wait_for_ongoing_refresh(&self) -> Result<EveJwtKeys, EsiError> {
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
        Err(EsiError::OAuthError(OAuthError::JwtKeyCacheError(
            error_message,
        )))
    }

    /// Checks if the cache has valid keys and triggers background refresh if needed
    /// Returns the keys if they are valid, otherwise returns None
    pub async fn check_cache_and_trigger_background_refresh(&self) -> Option<EveJwtKeys> {
        // Retrieve keys from cache
        let keys = {
            let cache = self.client.jwt_keys_cache.read().await;
            match &*cache {
                Some((keys, timestamp)) => Some((keys.clone(), *timestamp)),
                None => None,
            }
        };

        if let Some((keys, timestamp)) = keys {
            debug!("JWT keys found in cache");

            // Check if we should run a background refresh task
            let is_approaching_expiry = self.is_approaching_expiry(timestamp.elapsed().as_secs());

            if is_approaching_expiry {
                // Check if we should respect a backoff period due to previous failure
                let should_respect_backoff = self.should_respect_backoff().await;

                // Only trigger background refresh if not in backoff period and we can acquire the lock
                if !should_respect_backoff && self.try_acquire_refresh_lock() {
                    self.trigger_background_jwt_refresh().await;
                }
            }

            // Return keys if cache is not expired
            if !self.is_cache_expired(timestamp.elapsed().as_secs()) {
                debug!("Using cached JWT keys");
                return Some(keys);
            } else {
                debug!("JWT keys cache expired");
            }
        } else {
            debug!("JWT keys cache miss");
        }

        None
    }

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

    /// Determines if the cache is approaching expiry
    pub fn is_approaching_expiry(&self, elapsed_seconds: u64) -> bool {
        elapsed_seconds
            > (self.client.jwt_keys_cache_ttl * DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT
                / 100)
    }

    /// Determines if the cache is expired
    pub fn is_cache_expired(&self, elapsed_seconds: u64) -> bool {
        elapsed_seconds >= self.client.jwt_keys_cache_ttl
    }

    /// Checks if we should respect backoff period due to previous failure
    pub async fn should_respect_backoff(&self) -> bool {
        match &*self.client.jwt_keys_last_refresh_failure.read().await {
            Some(last_failure) => {
                last_failure.elapsed().as_secs() < DEFAULT_JWK_BACKGROUND_REFRESH_BACKOFF
            }
            None => false,
        }
    }

    /// Updates the JWT keys cache with new keys and the current timestamp
    pub async fn update_jwt_keys_cache(&self, keys: EveJwtKeys) {
        let mut cache = self.client.jwt_keys_cache.write().await;
        *cache = Some((keys, std::time::Instant::now()));
    }

    /// Releases the JWT key refresh lock and notifies any waiting threads
    pub fn release_refresh_lock_and_notify(&self) {
        debug!("Releasing JWT key refresh lock");
        self.client
            .jwt_key_refresh_in_progress
            .store(false, std::sync::atomic::Ordering::Release);

        // Notify waiters
        self.client.jwt_key_refresh_notifier.notify_waiters();
    }

    /// Records a JWT key refresh failure
    pub async fn record_refresh_failure(&self, attempt_count: u64) -> EsiError {
        let mut failure_time = self.client.jwt_keys_last_refresh_failure.write().await;
        *failure_time = Some(std::time::Instant::now());

        let error_message = format!("Failed to fetch JWT keys after {} attempts", attempt_count);

        error!("{}", error_message);

        EsiError::OAuthError(OAuthError::JwtKeyCacheError(error_message))
    }
}
