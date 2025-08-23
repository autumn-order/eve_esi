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
                    #[cfg(not(tarpaulin_include))]
                    debug!(
                        "Respecting backoff period: {}s elapsed of {}s required",
                        elapsed_secs, DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN
                    );
                } else {
                    #[cfg(not(tarpaulin_include))]
                    trace!(
                        "Backoff period elapsed: {}s passed (required {}s)",
                        elapsed_secs,
                        DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN
                    );
                }

                should_backoff
            }
            None => {
                #[cfg(not(tarpaulin_include))]
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
        let esi_client = self.client;

        // Retrieve cache TTL, this determines how many seconds it takes for the keys to expire
        // By default, it is 3600 seconds (1 hour)
        let jwt_cache_ttl = esi_client.jwt_keys_cache_ttl;

        // Determine how many seconds need to pass for the keys to be considered nearing expiration
        // By default, 80% of 3600 seconds must have elapsed, 2880 seconds.
        let threshold_seconds =
            jwt_cache_ttl * DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT / 100;

        // By default, if more than 2880 seconds have elapsed then the keys are nearing expiration.
        let is_approaching_expiry = elapsed_seconds > threshold_seconds;

        if is_approaching_expiry {
            #[cfg(not(tarpaulin_include))]
            debug!(
                "JWT keys cache approaching expiry: elapsed={}s, threshold={}s ({}% of ttl={}s)",
                elapsed_seconds,
                threshold_seconds,
                DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
                self.client.jwt_keys_cache_ttl
            );
        } else {
            #[cfg(not(tarpaulin_include))]
            trace!(
                "JWT keys cache still fresh: elapsed={}s, threshold={}s ({}% of ttl={}s)",
                elapsed_seconds,
                threshold_seconds,
                DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
                self.client.jwt_keys_cache_ttl
            );
        }

        is_approaching_expiry
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
            #[cfg(not(tarpaulin_include))]
            debug!(
                "JWT keys cache expired: elapsed={}s, ttl={}s",
                elapsed_seconds, self.client.jwt_keys_cache_ttl
            );
        } else {
            #[cfg(not(tarpaulin_include))]
            trace!(
                "JWT keys cache valid: elapsed={}s, ttl={}s",
                elapsed_seconds,
                self.client.jwt_keys_cache_ttl
            );
        }

        is_expired
    }
}

#[cfg(test)]
mod should_respect_backoff_tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::EsiClient;

    /// Validate backoff period is respected correctly
    ///
    /// When there is a backoff period within the default of the past (60 seconds),
    /// assert that the function returns true, indicating that we should not attempt
    /// a refresh.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Set the backoff period to within the default of 60 seconds
    ///
    /// # Assertions
    /// - Verifies that the function returns true, indicating that we should respect the backoff
    ///   period and not attempt a refresh.
    #[tokio::test]
    async fn test_should_respect_backoff_recent_failure() {
        // Setup EsiClient
        let mut esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set the backoff period to within default of (60 seconds)
        esi_client.jwt_keys_last_refresh_failure = Arc::new(RwLock::new(Some(
            std::time::Instant::now() - std::time::Duration::from_secs(30),
        )));

        // Run function
        let should_backoff = esi_client.oauth2().should_respect_backoff().await;

        // Assert true
        assert_eq!(should_backoff, true);
    }

    /// Validate that the backoff period is respected correctly when a past failure exists
    ///
    /// When the back off period is greater than the default of 60 seconds,
    /// assert that the function returns false, indicating that we can attempt a refresh.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Set the backoff period to greater than the default of 60 seconds
    ///
    /// # Assertions
    /// - Verifies that the function returns false, indicating that we can attempt a refresh.
    #[tokio::test]
    async fn test_should_respect_backoff_past_backoff() {
        // Setup EsiClient
        let mut esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set the back off period greater than default of (60 seconds)
        esi_client.jwt_keys_last_refresh_failure = Arc::new(RwLock::new(Some(
            std::time::Instant::now() - std::time::Duration::from_secs(61),
        )));

        // Run function
        let should_backoff = esi_client.oauth2().should_respect_backoff().await;

        // Assert false
        assert_eq!(should_backoff, false);
    }

    /// Validate that no backoff is needed when no past failure exists
    ///
    /// When there is no previous failure recorded, the function should return false,
    /// indicating that we can attempt a refresh without any backoff period.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Do not set any backoff period
    ///
    /// # Assertions
    /// - Verifies that the function returns false, indicating that we can attempt a refresh.
    #[tokio::test]
    async fn test_should_respect_backoff_no_failure() {
        // Setup EsiClient
        // Don't set back off period
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Run function
        let should_backoff = esi_client.oauth2().should_respect_backoff().await;

        // Assert false
        assert_eq!(should_backoff, false);
    }
}

#[cfg(test)]
mod is_approaching_expiry_tests {
    use crate::EsiClient;

    /// Validates function returns true if cache is approaching expiration
    ///
    /// When the JWT key cache expiration is past 80% expired (2880 seconds of 3600 default expiration),
    /// the function should return true indicating that the cache is almost expired.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Set the JWT key cache to beyond 80% expired
    ///
    /// # Validations
    /// - Verifies the function returns true, cache is almost expired.
    #[test]
    fn test_is_approaching_expiry_true() {
        // Setup EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set JWT key cache TTL to past 80% expired
        let expiry = std::time::Instant::now() - std::time::Duration::from_secs(3000);
        let timestamp = expiry.elapsed().as_secs();

        // Test function
        let result = esi_client.oauth2().is_approaching_expiry(timestamp);

        // Assert true
        assert_eq!(result, true)
    }

    /// Validates function returns false if cache is not approaching expiration
    ///
    /// When the JWT key cache expiration is not yet at 80%, the function
    /// should return false indicating we are not yet nearing expiration.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Set the client JWT key cache to less than 80% expired
    ///
    /// # Validations
    /// - Verifies the function returns false, cache is not yet nearing expiration.
    #[test]
    fn test_is_approaching_expiry_false() {
        // Setup EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set JWT key cache TTL to new keys
        let expiry = std::time::Instant::now();
        let timestamp = expiry.elapsed().as_secs();

        // Test function
        let result = esi_client.oauth2().is_approaching_expiry(timestamp);

        // Assert false
        assert_eq!(result, false)
    }
}
