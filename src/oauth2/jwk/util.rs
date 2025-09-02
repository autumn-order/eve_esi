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

use std::sync::Arc;

use log::{debug, trace};
use tokio::sync::RwLock;

/// Checks if refresh is still in cooldown due to recent failure.
///
/// This method determines whether enough time has passed since the last
/// JWT key refresh failure to attempt another refresh. It implements a
/// simple backoff mechanism to prevent excessive API calls when the
/// authentication service is experiencing issues.
///
/// # Thread Safety
/// This method acquires a read lock on the failure timestamp, allowing
/// multiple threads to check the backoff status concurrently.
/// - Reads from the shared [`EsiClient::jwt_keys_last_refresh_failure`](crate::EsiClient::jwt_keys_last_refresh_failure)
///   timestamp
///
/// # Arguments
/// - `jwk_refresh_cooldown` ([`u64`]): Cooldown in seconds between background refresh
///   attempts as defined by the [`OAuthConfig::jwk_refresh_cooldown`](crate::oauth2::OAuth2Config::jwk_refresh_cooldown)
///   field used by the [`EsiClient`](crate::EsiClient). By default this is 60 seconds.
/// - `jwt_key_last_refresh_failure` ([`EsiClient::jwt_keys_last_refresh_failure`](crate::EsiClient::jwt_keys_last_refresh_failure)):
///   field representing the last failed JWT key refresh attempt.
///
/// # Returns
/// - Some([`u64`]): Indicating the JWT key refresh cooldown remaining
/// - None: If there is no remaining JWT key refresh cooldown.
pub(super) async fn check_refresh_cooldown(
    jwk_refresh_cooldown: u64,
    jwt_key_last_refresh_failure: &Arc<RwLock<Option<std::time::Instant>>>,
) -> Option<u64> {
    // Check for last background refresh failure
    let last_refresh_failure = jwt_key_last_refresh_failure;
    if let Some(last_failure) = *last_refresh_failure.read().await {
        // Check if last refresh failure is within backoff period
        let elapsed_secs = last_failure.elapsed().as_secs();
        let is_cooldown = elapsed_secs < jwk_refresh_cooldown;

        if is_cooldown {
            #[cfg(not(tarpaulin_include))]
            debug!(
                "Respecting background refresh cooldown: {}s elapsed of {}s required",
                elapsed_secs, jwk_refresh_cooldown
            );

            // Return Some with the remaining cooldown in seconds
            let remaining_cooldown = jwk_refresh_cooldown - elapsed_secs;

            return Some(remaining_cooldown);
        } else {
            #[cfg(not(tarpaulin_include))]
            trace!(
                "Background cooldown period elapsed: {}s passed (required {}s)",
                elapsed_secs,
                jwk_refresh_cooldown
            );

            // Return None indicating there is no active cooldown
            return None;
        }
    }

    // No previous JWT key refresh failure
    trace!("No previous JWT key refresh failures recorded, no backoff needed");

    // Return None indicating there is no active cooldown
    return None;
}

/// Determines if the cache is approaching expiry based on elapsed time
///
/// Checks whether the elapsed time since the last cache update has crossed
/// the nearing expiration threshold percentage of the cache lifetime, indicating that a proactive
/// refresh should be triggered.
///
/// # Parameters
/// - `jwt_key_cache_ttl` ([`u64`]): Lifetime in seconds before cache is considered expired which
///   is defined by the [`OAuthConfig::jwk_cache_ttl`](crate::oauth2::OAuth2Config::jwk_cache_ttl)
///   field used by the [`EsiClient`](crate::EsiClient). By default this 3600 seconds
///   representing 1 hour.
/// - `background_refresh_threshold` ([`u64`]): Number representing % when cache is considered
///   nearing expiry which is defined by the
///   [`OAuthConfig::jwk_background_refresh_threshold_percent`](crate::oauth2::OAuth2Config::jwk_background_refresh_threshold_percent)
///   field used by the [`EsiClient`](crate::EsiClient) which represents the percentage of the total
///   TTL after which we consider the cache to be approaching expiry. By default this is 80%.
/// - `elapsed_seconds` ([`u64`]): Number of seconds since the cache was last updated
///
/// # Returns
/// - `true` if the elapsed time exceeds the threshold percentage of the TTL
/// - `false` if the cache is still well within its valid period
pub(super) fn is_cache_approaching_expiry(
    jwt_key_cache_ttl: u64,
    background_refresh_threshold: u64,
    elapsed_seconds: u64,
) -> bool {
    // Retrieve cache TTL, this determines how many seconds it takes for the keys to expire
    // By default, it is 3600 seconds (1 hour)
    let jwt_cache_ttl = jwt_key_cache_ttl;

    // Determine how many seconds need to pass for the keys to be considered nearing expiration
    // By default, 80% of 3600 seconds must have elapsed, 2880 seconds.
    let threshold_percentage = background_refresh_threshold / 100;
    let threshold_seconds = jwt_cache_ttl * threshold_percentage;

    // By default, if more than 2880 seconds have elapsed then the keys are nearing expiration.
    let is_approaching_expiry = elapsed_seconds > threshold_seconds;

    if is_approaching_expiry {
        #[cfg(not(tarpaulin_include))]
        debug!(
            "JWT keys cache approaching expiry: elapsed={}s, threshold={}s ({}% of ttl={}s)",
            elapsed_seconds, threshold_seconds, background_refresh_threshold, jwt_key_cache_ttl
        );

        // Return true if cache is approaching expiry
        true
    } else {
        #[cfg(not(tarpaulin_include))]
        trace!(
                "JWT keys cache not yet approaching expiry: elapsed={}s, threshold={}s ({}% of ttl={}s)",
                elapsed_seconds,
                threshold_seconds,
                background_refresh_threshold,
                jwt_key_cache_ttl
            );

        // Return false if cache is not yet approaching expiry
        false
    }
}

/// Determines if the cache has completely expired based on elapsed time
///
/// Checks if the elapsed time since the last cache update has reached or
/// exceeded the configured JWT key cache lifetime which indicates expiration.
///
/// # Parameters
/// - `jwt_key_cache_ttl` ([`u64`]): Lifetime in seconds before cache is considered expired
///   which is defined by the
///   [`OAuthConfig::jwk_cache_ttl`](crate::oauth2::OAuth2Config::jwk_cache_ttl)
///   field used by [`EsiClient`](crate::EsiClient). By default this is 3600 seconds
///   representing 1 hour.
/// - `elapsed_seconds` ([`u64`]): Number of seconds since the cache was last updated
///
/// # Returns
/// - `true` if the elapsed time has reached or exceeded the TTL
/// - `false` if the cache is still within its valid period
pub(super) fn is_cache_expired(jwt_key_cache_ttl: u64, elapsed_seconds: u64) -> bool {
    let is_expired = elapsed_seconds >= jwt_key_cache_ttl;

    if is_expired {
        #[cfg(not(tarpaulin_include))]
        debug!(
            "JWT keys cache expired: elapsed={}s, ttl={}s",
            elapsed_seconds, jwt_key_cache_ttl
        );

        // Return true if cache is not yet expired
        true
    } else {
        #[cfg(not(tarpaulin_include))]
        trace!(
            "JWT keys cache valid: elapsed={}s, ttl={}s",
            elapsed_seconds,
            jwt_key_cache_ttl
        );

        // Return false if cache is still valid
        false
    }
}

#[cfg(test)]
mod is_refresh_cooldown_tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::EsiClient;

    use super::check_refresh_cooldown;

    /// Refresh cooldown should be active due to recent failure
    ///
    /// When there is a refresh failure within the default of the past (60 seconds),
    /// assert that the function returns true, indicating that we should not attempt
    /// a refresh due to cooldown period.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Set last refresh failure within default cooldown period of past 60 seconds
    ///
    /// # Assertions
    /// - Assert function returns Some indicating we are still in cooldown
    /// - Assert 30 seconds remain in cooldown period
    #[tokio::test]
    async fn test_check_refresh_cooldown_within_cooldown() {
        // Setup EsiClient
        let mut esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set the recent failure within cooldown period default of 60 seconds
        esi_client.jwt_key_last_refresh_failure = Arc::new(RwLock::new(Some(
            std::time::Instant::now() - std::time::Duration::from_secs(30),
        )));

        // Run function
        let cooldown = check_refresh_cooldown(
            esi_client.oauth2_config.jwk_refresh_cooldown,
            &esi_client.jwt_key_last_refresh_failure,
        )
        .await;

        // Assert cooldown is some
        assert!(cooldown.is_some());
        let remaining_cooldown = cooldown.unwrap();

        // Assert cooldown returns expected 30 seconds remaining
        assert_eq!(remaining_cooldown, 30);
    }

    /// Refresh cooldown should be false due to not being in cooldown
    ///
    /// When the back off period is greater than the default of 60 seconds,
    /// assert that the function returns false, indicating that we can attempt a refresh.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Set last refresh failure beyond default cooldown period of past 60 seconds
    ///
    /// # Assertions
    /// - Assert cooldown is None indicating we are not in the cooldown period
    #[tokio::test]
    async fn test_check_refresh_cooldown_recent_failure() {
        // Setup EsiClient
        let mut esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set the last refresh failure greater than default of cooldown period of 60 seconds
        esi_client.jwt_key_last_refresh_failure = Arc::new(RwLock::new(Some(
            std::time::Instant::now() - std::time::Duration::from_secs(61),
        )));

        // Run function
        let cooldown = check_refresh_cooldown(
            esi_client.oauth2_config.jwk_refresh_cooldown,
            &esi_client.jwt_key_last_refresh_failure,
        )
        .await;

        // Assert cooldown is None
        assert!(cooldown.is_none());
    }

    /// Refresh cooldown should be false due to no past failures recorded
    ///
    /// When there is no previous failure recorded, the function should return false,
    /// indicating that we can attempt a refresh.
    ///
    /// # Test Setup
    /// - Create a basic EsiClient
    /// - Do not set the [`EsiClient::jwt_key_last_refresh_failure`]
    ///
    /// # Assertions
    /// - Assert cooldown is None indicating we are not in the cooldown period
    #[tokio::test]
    async fn test_check_refresh_cooldown_no_failure() {
        // Setup EsiClient
        // Don't set back off period
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Run function
        let cooldown = check_refresh_cooldown(
            esi_client.oauth2_config.jwk_refresh_cooldown,
            &esi_client.jwt_key_last_refresh_failure,
        )
        .await;

        // Assert cooldown is None
        assert!(cooldown.is_none());
    }
}

#[cfg(test)]
mod is_cache_approaching_expiry_tests {
    use crate::EsiClient;

    use super::is_cache_approaching_expiry;

    /// Validates function returns true if cache is past 80% expiration
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
    fn test_is_cache_approaching_expiry_true() {
        // Setup EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set the expiration timestamp to psat default expiry of 2880 seconds
        // Default approaching expiry is 2880 seconds (80% of 3600 seconds default)
        let timestamp = std::time::Instant::now() - std::time::Duration::from_secs(2881);
        let elapsed_seconds = timestamp.elapsed().as_secs();

        // Test function
        let result = is_cache_approaching_expiry(
            esi_client.oauth2_config.jwk_cache_ttl,
            esi_client
                .oauth2_config
                .jwk_background_refresh_threshold_percent,
            elapsed_seconds,
        );

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
    fn test_is_cache_approaching_expiry_false() {
        // Setup EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@email.com")
            .build()
            .expect("Failed to build EsiClient");

        // Set the expiration timestamp to represent fresh keys
        let timestamp = std::time::Instant::now();
        let elapsed_seconds = timestamp.elapsed().as_secs();

        // Test function
        let result = is_cache_approaching_expiry(
            esi_client.oauth2_config.jwk_cache_ttl,
            esi_client
                .oauth2_config
                .jwk_background_refresh_threshold_percent,
            elapsed_seconds,
        );

        // Assert false
        assert_eq!(result, false)
    }
}

#[cfg(test)]
mod is_cache_expired_tests {
    use crate::EsiClient;

    use super::is_cache_expired;

    /// Validates function returns true if cache is expired
    ///
    /// When the JWT key cache has been set more than 3600 seconds ago
    /// by default, the cache should be considered fully expired.
    ///
    /// # Setup
    /// - Create a basic EsiClient
    /// - Set the client JWT key cache to past 3600 seconds expiration
    ///
    /// # Assertions
    /// - Verifies the function returns true, the cache is fully expired
    #[test]
    fn test_is_cache_expired_true() {
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Set expiration timestamp to past default expiration of 3600 seconds
        let timestamp = std::time::Instant::now() - std::time::Duration::from_secs(3601);
        let elapsed_seconds = timestamp.elapsed().as_secs();

        // Test function
        let result = is_cache_expired(esi_client.oauth2_config.jwk_cache_ttl, elapsed_seconds);

        // Assert true
        assert_eq!(result, true)
    }

    /// Validates function returns false if cache is not expired
    ///
    /// When the JWT key cache has been set less than 3600 seconds ago
    /// by default, the cache should be not yet expired.
    ///
    /// # Setup
    /// - Create a basic EsiClient
    /// - Set the client JWT key cache to fresh keys
    ///
    /// # Assertions
    /// - Verifies the function returns true, the cache is fully expired
    #[test]
    fn test_is_cache_expired_false() {
        // Setup basic EsiClient
        let esi_client = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        // Set expiration timestamp to represent fresh keys
        let timestamp = std::time::Instant::now();
        let elapsed_seconds = timestamp.elapsed().as_secs();

        // Test function
        let result = is_cache_expired(esi_client.oauth2_config.jwk_cache_ttl, elapsed_seconds);

        // Assert true
        assert_eq!(result, false)
    }
}
