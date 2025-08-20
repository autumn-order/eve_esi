//! JWT Key Utility Functions
//!
//! This module provides utility functions for JWT key management, including:
//!
//! - Cache expiry calculations
//! - Backoff period enforcement
//! - Error handling and reporting
//!
//! These utilities support the core JWT key operations with helper functions
//! that implement common patterns and checks used throughout the JWT key system.
//!
//! See the [module-level documentation](super) for a more detailed overview and usage.

use log::{debug, trace};

use crate::constant::{
    DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN, DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
};
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Checks if we should respect backoff period due to previous failure
    ///
    /// This method determines whether enough time has passed since the last
    /// JWT key refresh failure to attempt another refresh. It implements a
    /// simple backoff mechanism to prevent excessive API calls when the
    /// authentication service is experiencing issues.
    ///
    /// # Implementation Details
    /// - Reads from the shared [`crate::EsiClient::jwt_keys_last_refresh_failure`] timestamp
    /// - Uses [`DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN`] (60 seconds) as the minimum wait time
    ///   between refresh attempts after a failure
    ///
    /// # Thread Safety
    /// This method acquires a read lock on the failure timestamp, allowing
    /// multiple threads to check the backoff status concurrently.
    ///
    /// # Returns
    /// - `true` if we are still within the backoff period and should not attempt another refresh
    /// - `false` if either no previous failure exists or the backoff period has elapsed
    pub(super) async fn should_respect_backoff(&self) -> bool {
        match &*self.client.jwt_keys_last_refresh_failure.read().await {
            Some(last_failure) => {
                let elapsed_secs = last_failure.elapsed().as_secs();
                let should_backoff = elapsed_secs < DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN;

                if should_backoff {
                    debug!(
                        "Respecting backoff period: {}s elapsed of {}s required",
                        elapsed_secs, DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN
                    );
                } else {
                    trace!(
                        "Backoff period elapsed: {}s passed (required {}s)",
                        elapsed_secs,
                        DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN
                    );
                }

                should_backoff
            }
            None => {
                trace!("No previous JWT key refresh failures recorded, no backoff needed");
                false
            }
        }
    }

    /// Determines if the cache is approaching expiry based on elapsed time
    ///
    /// Checks whether the elapsed time since the last cache update has crossed
    /// the threshold percentage of the total TTL, indicating that a proactive
    /// refresh should be triggered.
    ///
    /// # Implementation Details
    /// The threshold is defined by [`DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT`] (80%),
    /// which represents the percentage of the total TTL after which we consider
    /// the cache to be approaching expiry.
    ///
    /// # Parameters
    /// - `elapsed_seconds`: Number of seconds since the cache was last updated
    ///
    /// # Returns
    /// - `true` if the elapsed time exceeds the threshold percentage of the TTL
    /// - `false` if the cache is still well within its valid period
    pub(super) fn is_approaching_expiry(&self, elapsed_seconds: u64) -> bool {
        let threshold_seconds =
            self.client.jwt_keys_cache_ttl * DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT / 100;
        let is_approaching = elapsed_seconds > threshold_seconds;

        if is_approaching {
            debug!(
                "JWT keys cache approaching expiry: elapsed={}s, threshold={}s ({}% of ttl={}s)",
                elapsed_seconds,
                threshold_seconds,
                DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
                self.client.jwt_keys_cache_ttl
            );
        } else {
            trace!(
                "JWT keys cache still fresh: elapsed={}s, threshold={}s ({}% of ttl={}s)",
                elapsed_seconds,
                threshold_seconds,
                DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
                self.client.jwt_keys_cache_ttl
            );
        }

        is_approaching
    }

    /// Determines if the cache has completely expired based on elapsed time
    ///
    /// Checks if the elapsed time since the last cache update has reached or
    /// exceeded the configured TTL (default: 3600 seconds/1 hour), indicating
    /// that the cached keys should no longer be used.
    ///
    /// # Parameters
    /// - `elapsed_seconds`: Number of seconds since the cache was last updated
    ///
    /// # Returns
    /// - `true` if the elapsed time has reached or exceeded the TTL
    /// - `false` if the cache is still within its valid period
    ///
    /// # Related Methods
    ///
    /// ## Utility
    /// - [`Self::is_approaching_expiry`]: Checks if the cache is nearing expiration
    ///   but hasn't fully expired yet
    pub(super) fn is_cache_expired(&self, elapsed_seconds: u64) -> bool {
        let is_expired = elapsed_seconds >= self.client.jwt_keys_cache_ttl;

        if is_expired {
            debug!(
                "JWT keys cache expired: elapsed={}s, ttl={}s",
                elapsed_seconds, self.client.jwt_keys_cache_ttl
            );
        } else {
            trace!(
                "JWT keys cache valid: elapsed={}s, ttl={}s",
                elapsed_seconds,
                self.client.jwt_keys_cache_ttl
            );
        }

        is_expired
    }
}
