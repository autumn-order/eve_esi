//! # JWT Key Cache Builder
//!
//! Provides methods to override the default JWT key cache settings for the eve_esi crate. This allows the
//! modification of the JWT key endpoint URL and the logic of how JWT key caching and refreshing is
//! handled.
//!
//! ## Features
//! - Override JWT token key endpoint URL
//! - Adjust expiration times for JWT key cache
//! - Adjust the timeout for waiting for JWT key refreshes
//! - Adjust wait time/backoff period beteween refresh and how many retries should be made to fetch JWT keys
//! - Enable/disable the proactive background JWT key refresh
//!
//! ## Builder Methods
//!
//! | Method          | Purpose                                    |
//! | --------------- | ------------------------------------------ |
//! | `new`           | Create a new OAuth2Config builder          |
//! | `build`         | Build the OAuth2 config                    |
//! | `jwk_url`       | URL for EVE OAuth2 token keys              |
//! | `cache_ttl`     | The time that JWT keys are cached for      |
//! | `refresh_max_retries` | Amount of retries when a key fetch fails |
//! | `refresh_backoff`     | How long to wait between retries         |
//! | `refresh_timeout`     | How long to wait for another thread to refresh |
//! | `refresh_cooldown`    | Cooldown between sets of JWT key refresh attempts |
//! | `background_refresh_enabled` | Enable/disable background refresh          |
//! | `background_refresh_threshold` | Percentage at which cache is refreshed proactively |
//!
//! ## Usage
//!
//! ```
//! use eve_esi::EsiClient;
//! use eve_esi::oauth2::jwk::JwtKeyCache;
//!
//! // Build a cache with custom settings
//! let cache = JwtKeyCache::builder()
//!     // Set cache lifetime to 7200 seconds representing 2 hours
//!     .cache_ttl(7200)
//!     .build()
//!     .expect("Failed to build JwtKeyCache");
//!
//! // Apply cache settings to EsiClient
//! let esi_client = EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com")
//!     .jwt_key_cache(cache)
//!     .build()
//!     .expect("Failed to build EsiClient");
//! ```

use std::sync::atomic::AtomicBool;

use tokio::sync::{Notify, RwLock};

use crate::{
    constant::{
        DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT, DEFAULT_JWK_CACHE_TTL,
        DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_COOLDOWN, DEFAULT_JWK_REFRESH_MAX_RETRIES,
        DEFAULT_JWK_REFRESH_TIMEOUT, DEFAULT_JWK_URL,
    },
    error::EsiError,
    oauth2::error::OAuthConfigError,
    oauth2::jwk::cache::JwtKeyCache,
};

/// Builder struct for configuring & constructing a [`JwtKeyCache`]
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct JwtKeyCacheBuilder {
    // Cache Settings
    /// JWT key cache lifetime before expiration in seconds (3600 seconds representing 1 hour)
    pub(crate) cache_ttl: u64,

    // Refresh Settings
    /// JSON web token key URL that provides keys used to validate tokens
    pub(crate) jwk_url: String,
    /// Maximum number of retries for JWT key refresh when cache is empty or expired (default 2 retries)
    pub(crate) refresh_max_retries: u64,
    /// Backoff period in seconds after a JWT key refresh failure when cache is empty or expired (default 100 milliseconds)
    pub(crate) refresh_backoff: u64,
    /// Timeout in seconds when waiting for another thread to refresh JWT key (default 5 seconds)
    pub(crate) refresh_timeout: u64,
    /// Cooldown period in seconds after a failed set of JWT key refresh attempts (default 60 seconds)
    pub(crate) refresh_cooldown: u64,

    // Background Refresh Settings
    /// Determines whether or not a background task is spawned to refresh JWT keys nearing expiration proactively
    pub(crate) background_refresh_enabled: bool,
    /// Percentage of jwk_cache_ttl for when the background JWT key refresh is triggered (default 80%)
    pub(crate) background_refresh_threshold: u64,
}

impl JwtKeyCacheBuilder {
    /// Creates a new [`JwtKeyCacheBuilder`] instance used to build an [`JwtKeyCache`]
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`JwtKeyCacheBuilder`]: Instance with the default settings that can be overridden with setter methods.
    pub fn new() -> Self {
        Self {
            // Cache Settings
            cache_ttl: DEFAULT_JWK_CACHE_TTL,

            // Refresh Settings
            jwk_url: DEFAULT_JWK_URL.to_string(),
            refresh_max_retries: DEFAULT_JWK_REFRESH_MAX_RETRIES,
            refresh_backoff: DEFAULT_JWK_REFRESH_BACKOFF,
            refresh_timeout: DEFAULT_JWK_REFRESH_TIMEOUT,
            refresh_cooldown: DEFAULT_JWK_REFRESH_COOLDOWN,

            // Background Refresh Settings
            background_refresh_enabled: true,
            background_refresh_threshold: DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
        }
    }

    /// Builds a [`JwtKeyCache`] instance
    ///
    /// Converts an [`JwtKeyCacheBuilder`] into a [`JwtKeyCache`] with the configured values that
    /// were set with the builder methods.
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`JwtKeyCache`]: instance with the settings configured on the builder
    ///
    /// # Errors
    /// - [`EsiError`]: If the `background_refresh_threshold` is configured incorrectly.
    pub fn build(self) -> Result<JwtKeyCache, EsiError> {
        // Ensure background refresh percentage is set properly
        if !(self.background_refresh_threshold > 0) {
            return Err(EsiError::OAuthConfigError(
                OAuthConfigError::InvalidBackgroundRefreshThreshold,
            ));
        }
        if !(self.background_refresh_threshold < 100) {
            return Err(EsiError::OAuthConfigError(
                OAuthConfigError::InvalidBackgroundRefreshThreshold,
            ));
        }

        Ok(JwtKeyCache {
            cache: RwLock::new(None),
            refresh_lock: AtomicBool::new(false),
            refresh_notifier: Notify::new(),
            last_refresh_failure: RwLock::new(None),

            // Cache Settings
            cache_ttl: self.cache_ttl,

            // Refresh Settings
            jwk_url: self.jwk_url,
            refresh_max_retries: self.refresh_max_retries,
            refresh_backoff: self.refresh_backoff,
            refresh_timeout: self.refresh_timeout,
            refresh_cooldown: self.refresh_cooldown,

            // Background Refresh Settings
            background_refresh_enabled: self.background_refresh_enabled,
            background_refresh_threshold: self.background_refresh_threshold,
        })
    }

    /// Sets the EVE Online JWK URI to a custom URL.
    ///
    /// This method configures the JWK URI for EVE Online OAuth2 to a custom URL.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `jwk_url` (&[`str`]): The EVE Online JWK URL.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with updated EVE Online JWK URI configuration.
    pub fn jwk_url(mut self, jwk_url: &str) -> Self {
        self.jwk_url = jwk_url.to_string();
        self
    }

    /// Modifies the default lifetime of the JWT keys stored in cache
    ///
    /// By default, JWT keys are stored in cache for 3600 seconds (1 hour)
    /// before they are considered expired and need to be refreshed.
    ///
    /// Additionally, JWT keys are proactively refreshed by a background
    /// task at 80% expiration. You may wish to modify the
    /// [`Self::jwk_background_refresh_threshold_percent`] or disable the
    /// background refresh altogether with [`Self::jwk_background_refresh_enabled`].
    ///
    /// # Arguments
    /// - `seconds` ([`u64`]): The lifetime in seconds of the JWT keys stored in the cache.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the updated JWT key cache TTL
    pub fn cache_ttl(mut self, seconds: u64) -> Self {
        self.cache_ttl = seconds;
        self
    }

    /// Modifies the max amount of refresh attempts when fetching JWT keys
    ///
    /// This determines how many attempts are made to refresh JWT keys when
    /// the cache is empty or fully expired and it is imperative to refresh the
    /// cache in order to validate tokens.
    ///
    /// Between each fetch attempt there is an exponential backoff of 100ms by default
    /// which can be modified with the [`Self::jwk_refresh_backoff`] method.
    ///
    /// This does not affect the background JWT key refresh which is instead modified
    /// with the [`Self::jwk_background_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `retry_attempts` ([`u64`]): The amount of retry attempts if a JWT key fetch fails.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the updated JWK refresh max retries
    pub fn refresh_max_retries(mut self, retry_attempts: u64) -> Self {
        self.refresh_max_retries = retry_attempts;
        self
    }

    /// Modifies the exponential backoff duration in milliseconds between JWT key fetch retry attempts
    ///
    /// The default behavior is a 100ms exponential backoff between each retry attempt
    /// to refresh JWT keys when the cache is either empty or expired.
    ///
    /// For example: 100ms, 200ms, 400ms, etc
    ///
    /// The amount of retry attempts can be modified with the [`Self::jwk_refresh_max_retries`]
    /// method.
    ///
    /// This does not affect the background JWT key refresh which is instead modified
    /// with the [`Self::jwk_background_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `backoff_milliseconds` ([`u64`]): The exponential backoff duration in milliseconds between each attempt.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the updated exponential backoff
    pub fn refresh_backoff(mut self, backoff_milliseconds: u64) -> Self {
        self.refresh_backoff = backoff_milliseconds;
        self
    }

    /// Modifies the timeout waiting for another thread to perform a JWT key cache refresh
    ///
    /// This library uses a refresh lock shared between threads to indicate if a JWT key
    /// cache refresh is already in progress on another thread. If the JWT key cache is
    /// currently empty or expired and this refresh lock is in place, the current thread
    /// will wait for a default of 5 seconds before timing out if the refresh takes too long.
    ///
    /// # Arguments
    /// - `timeout_seconds` ([`u64`]): The timeout in seconds to wait for another thread to complete a
    ///   JWT key refresh.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the modified timeout setting.
    pub fn refresh_timeout(mut self, timeout_seconds: u64) -> Self {
        self.refresh_timeout = timeout_seconds;
        self
    }

    /// Modifies the cooldown between sets of JWT key cache refresh attempts in the event of failure
    ///
    /// By default, when a set of JWT key cache refresh attempts fail there will be a cooldown of 60 seconds
    /// between the next set of attempts to refresh JWT keys before expiration.
    ///
    /// # Arguments
    /// - `cooldown_seconds` ([`u64`]): Cooldown in seconds between background JWT key cache refresh attempts.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the modified background refresh cooldown.
    pub fn refresh_cooldown(mut self, cooldown_seconds: u64) -> Self {
        self.refresh_cooldown = cooldown_seconds;
        self
    }

    /// Modifies whether or not the proactive background refresh when JWT keys are almost expired is enabled
    ///
    /// By default, when the JWT key cache is nearing expiration at around 80%, a background refresh task
    /// will be spawned to proactively refresh the keys. This behavior is thread safe and a more detailed
    /// description of how it works can be found at [`crate::oauth2::OAuth2Api::get_jwt_keys`].
    ///
    /// This functionality has been built with high volume applications in mind and will work for the
    /// vast majority of production use cases. In the instance where you do want to have more control
    /// over proactive JWT key refreshes consider disabling this and using a cron task to perform
    /// a refresh instead with the [`crate::oauth2::OAuth2Api::fetch_and_update_cache`] method which will
    /// update the cache regardless of expiration status.
    ///
    /// You can modify the % at which the proactive background refresh is triggered with the
    /// [`Self::jwk_background_refresh_threshold_percent`] method.
    ///
    /// # Arguments
    /// - `background_refresh_enabled` ([`bool`]): A bool indicating whether or not the background refresh
    ///   is enabled.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the modified status of whether or not the background refresh is enabled.
    pub fn background_refresh_enabled(mut self, background_refresh_enabled: bool) -> Self {
        self.background_refresh_enabled = background_refresh_enabled;
        self
    }

    /// The % of JWT key cache lifetime for when the proactive background JWT key cache refresh is triggered
    ///
    /// By default, when the JWT key cache reaches 80% of the default 3600 second cache lifetime, a proactive
    /// JWT key cache refresh will be triggered next time the [`crate::oauth2::OAuth2Api::get_jwt_keys`]
    /// method is called.
    ///
    /// Should the attempt fail, there will be a 60 second cooldown between attempts which can be modified with
    /// the [`Self::jwk_background_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `threshold_percent` ([`u64`]): A number representing the percentage of when the refresh should be triggered.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the modified proactive background refresh threshold percentage.
    pub fn background_refresh_threshold(mut self, threshold_percentage: u64) -> Self {
        self.background_refresh_threshold = threshold_percentage;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures that all setter methods for [`JwtKeyCacheBuilder`] work as expected
    ///
    /// Test Setup
    /// - Create a new instance of [`JwtKeyCacheBuilder`] and use each setter method
    /// - Build the [`JwtKeyCacheBuilder`] returning an [`JwtKeyCache`]
    ///
    /// Assertions
    /// - Assert cache settings were set as expected
    /// - Assert refresh settings were set as expected
    /// - Assert background refresh settings were set as expected
    #[test]
    fn test_config_setter_methods() {
        let cache = JwtKeyCacheBuilder::new()
            // Cache settings
            .cache_ttl(0)
            // Refresh settings
            .jwk_url("https://example.com")
            .refresh_max_retries(0)
            .refresh_backoff(0)
            .refresh_timeout(0)
            .refresh_cooldown(0)
            // Background refresh settings
            .background_refresh_enabled(false)
            .background_refresh_threshold(1)
            .build()
            .expect("Failed to build JwtKeyCache");

        // Assert cache settings were set
        assert_eq!(cache.cache_ttl, 0);

        // Assert refresh settings were set
        assert_eq!(cache.jwk_url, "https://example.com");
        assert_eq!(cache.refresh_max_retries, 0);
        assert_eq!(cache.refresh_backoff, 0);
        assert_eq!(cache.refresh_timeout, 0);
        assert_eq!(cache.refresh_cooldown, 0);

        // Assert background refresh settings were set
        assert_eq!(cache.background_refresh_enabled, false);
        assert_eq!(cache.background_refresh_threshold, 1);
    }

    /// Expect an error setting the background refresh threshold to 0
    ///
    /// # Test Setup
    /// - Attempt to build a [`JwtKeyCache`] with the background_refresh_threshold to 0
    ///
    /// # Assertions
    /// - Assert result is an error
    /// - Assert error is of type OAuthConfigError::InvalidBackgroundRefreshThreshold
    #[test]
    fn test_invalid_background_refresh_threshold_0() {
        // Create a JwtKeyCache with invalid threshold percent
        let result = JwtKeyCache::builder()
            .background_refresh_threshold(0)
            .build();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type OAuthConfigError::InvalidBackgroundRefreshThreshold
        assert!(matches!(
            result,
            Err(EsiError::OAuthConfigError(
                OAuthConfigError::InvalidBackgroundRefreshThreshold
            ))
        ))
    }

    /// Expect an error setting the background refresh threshold to 100
    ///
    /// # Test Setup
    /// - Attempt to build an [`JwtKeyCache`] with the background_refresh_threshold to 100
    ///
    /// # Assertions
    /// - Assert result is an error
    /// - Assert error is of type OAuthConfigError::InvalidBackgroundRefreshThreshold
    #[test]
    fn test_invalid_background_refresh_threshold_100() {
        // Create a JwtKeyCache with invalid threshold percent
        let result = JwtKeyCache::builder()
            .background_refresh_threshold(100)
            .build();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type OAuthConfigError::InvalidBackgroundRefreshThreshold
        assert!(matches!(
            result,
            Err(EsiError::OAuthConfigError(
                OAuthConfigError::InvalidBackgroundRefreshThreshold
            ))
        ))
    }
}
