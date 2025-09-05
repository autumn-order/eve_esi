//! # EVE Online ESI Client Config
//!
//! Provides methods to override defaults for the [`EsiClient`]. This allows the
//! modification of the OAuth2 endpoint URLs and the logic of how JWT key caching and refreshing is
//! handled.
//!
//! ## Features
//! - Override OAuth2 JWT key, access token, and authorization endpoint URLs
//! - Adjust expiration times for JWT key cache
//! - Adjust the timeout between sets of JWT key refresh attempts
//! - Adjust backoff period (wait time) beteween attempts and how many retries should be made to refresh JWT keys
//! - Enable/disable the proactive background JWT key refresh & change the threshold at which it triggers
//!
//! ## Builder Methods
//!
//! | Method          | Purpose                                    |
//! | --------------- | ------------------------------------------ |
//! | `new`           | Create a new [`EsiConfigBuilder`]          |
//! | `build`         | Build the [`EsiConfig`]                    |
//! | `auth_url`      | URL for sign in with EVE Online            |
//! | `token_url`     | URL to retrieve access tokens for OAuth2   |
//! | `jwk_url`       | URL for JWT keys to validate tokens        |
//! | `jwk_cache_ttl`     | The time that JWT keys are cached for      |
//! | `jwk_refresh_max_retries` | Amount of retries when a key fetch fails |
//! | `jwk_refresh_backoff`     | How long to wait between retries         |
//! | `jwk_refresh_timeout`     | How long to wait for another thread to refresh |
//! | `jwk_refresh_cooldown`    | Cooldown between sets of JWT key refresh attempts |
//! | `jwk_background_refresh_enabled` | Enable/disable background refresh          |
//! | `jwk_background_refresh_threshold` | Percentage at which cache is refreshed proactively |
//!
//! ## Usage
//!
//! ```
//! use eve_esi::EsiClient;
//! use eve_esi::config::EsiConfig;
//!
//! // Build a config to override defaults
//! let config = EsiConfig::builder()
//!     // Set cache lifetime to 7200 seconds representing 2 hours
//!     .jwk_cache_ttl(7200)
//!     .build()
//!     .expect("Failed to build EsiConfig");
//!
//! // Apply config settings to EsiClient
//! let esi_client = EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com")
//!     .config(config)
//!     .build()
//!     .expect("Failed to build EsiClient");
//! ```

use oauth2::{AuthUrl, TokenUrl};

use crate::{
    constant::{
        DEFAULT_AUTH_URL, DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT, DEFAULT_JWK_CACHE_TTL,
        DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_COOLDOWN, DEFAULT_JWK_REFRESH_MAX_RETRIES,
        DEFAULT_JWK_REFRESH_TIMEOUT, DEFAULT_JWK_URL, DEFAULT_TOKEN_URL,
    },
    error::EsiError,
    oauth2::error::OAuthConfigError,
};

/// Configuration settings for the [`EsiClient`]
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiConfig {
    // URL settings
    pub(crate) auth_url: AuthUrl,
    pub(crate) token_url: TokenUrl,
    /// JSON web token key URL that provides keys used to validate tokens
    pub(crate) jwk_url: String,

    // JWT Key Settings
    /// JWT key cache lifetime before expiration in seconds (3600 seconds representing 1 hour)
    pub(crate) jwk_cache_ttl: u64,
    /// Maximum number of retries for JWT key refresh when cache is empty or expired (default 2 retries)
    pub(crate) jwk_refresh_max_retries: u64,
    /// Backoff period in seconds after a JWT key refresh failure when cache is empty or expired (default 100 milliseconds)
    pub(crate) jwk_refresh_backoff: u64,
    /// Timeout in seconds when waiting for another thread to refresh JWT key (default 5 seconds)
    pub(crate) jwk_refresh_timeout: u64,
    /// Cooldown period in seconds after a failed set of JWT key refresh attempts (default 60 seconds)
    pub(crate) jwk_refresh_cooldown: u64,

    // JWT Key Background Refresh Settings
    /// Determines whether or not a background task is spawned to refresh JWT keys nearing expiration proactively
    pub(crate) jwk_background_refresh_enabled: bool,
    /// Percentage of jwk_cache_ttl for when the background JWT key refresh is triggered (default 80%)
    pub(crate) jwk_background_refresh_threshold: u64,
}

/// Builder struct for configuring & constructing an [`EsiConfig`] to override default settings
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiConfigBuilder {
    // URL settings
    pub(crate) auth_url: String,
    pub(crate) token_url: String,
    /// JSON web token key URL that provides keys used to validate tokens
    pub(crate) jwk_url: String,

    // JWT Key Settings
    /// JWT key cache lifetime before expiration in seconds (3600 seconds representing 1 hour)
    pub(crate) jwk_cache_ttl: u64,
    /// Maximum number of retries for JWT key refresh when cache is empty or expired (default 2 retries)
    pub(crate) jwk_refresh_max_retries: u64,
    /// Backoff period in seconds after a JWT key refresh failure when cache is empty or expired (default 100 milliseconds)
    pub(crate) jwk_refresh_backoff: u64,
    /// Timeout in seconds when waiting for another thread to refresh JWT key (default 5 seconds)
    pub(crate) jwk_refresh_timeout: u64,
    /// Cooldown period in seconds after a failed set of JWT key refresh attempts (default 60 seconds)
    pub(crate) jwk_refresh_cooldown: u64,

    // JWT Key Background Refresh Settings
    /// Determines whether or not a background task is spawned to refresh JWT keys nearing expiration proactively
    pub(crate) jwk_background_refresh_enabled: bool,
    /// Percentage of jwk_cache_ttl for when the background JWT key refresh is triggered (default 80%)
    pub(crate) jwk_background_refresh_threshold: u64,
}

impl EsiConfig {
    /// Creates a new instance of [`EsiConfig`] with default settings
    ///
    /// For details see [module-level documentation](self).
    ///
    /// # Returns
    /// - [`EsiConfig`]: with default settings
    ///
    /// # Errors
    /// - [`EsiError`]: If the [`EsiConfigBuilder::jwk_background_refresh_threshold`] is configured incorrectly.
    pub fn new() -> Result<Self, EsiError> {
        EsiConfigBuilder::new().build()
    }

    /// Returns a [`EsiConfigBuilder`] instance used to configure JWT key related settings
    ///
    /// Allows for the configuration of the [`EsiConfig`] using the [`EsiConfigBuilder`]
    /// setter methods to override the default configuration.
    ///
    /// For details see [module-level documentation](self).
    ///
    /// # Returns
    /// - [`EsiConfigBuilder`]: Instance with the default config which can be overridden with setter methods.
    pub fn builder() -> EsiConfigBuilder {
        EsiConfigBuilder::new()
    }
}

impl EsiConfigBuilder {
    /// Creates a new [`EsiConfigBuilder`] instance used to build an [`EsiConfig`]
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`EsiConfigBuilder`]: Instance with the default settings that can be overridden with setter methods.
    pub fn new() -> Self {
        Self {
            // URL Settings
            auth_url: DEFAULT_AUTH_URL.to_string(),
            token_url: DEFAULT_TOKEN_URL.to_string(),
            jwk_url: DEFAULT_JWK_URL.to_string(),

            // Refresh Settings
            jwk_cache_ttl: DEFAULT_JWK_CACHE_TTL,
            jwk_refresh_max_retries: DEFAULT_JWK_REFRESH_MAX_RETRIES,
            jwk_refresh_backoff: DEFAULT_JWK_REFRESH_BACKOFF,
            jwk_refresh_timeout: DEFAULT_JWK_REFRESH_TIMEOUT,
            jwk_refresh_cooldown: DEFAULT_JWK_REFRESH_COOLDOWN,

            // Background Refresh Settings
            jwk_background_refresh_enabled: true,
            jwk_background_refresh_threshold: DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
        }
    }

    /// Builds a [`EsiConfig`] instance
    ///
    /// Converts an [`EsiConfigBuilder`] into a [`EsiConfig`] with the configured values that
    /// were set with the builder methods.
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`EsiConfig`]: instance with the settings configured on the builder
    ///
    /// # Errors
    /// - [`EsiError`]: If the [`Self::jwk_background_refresh_threshold`] is configured incorrectly.
    pub fn build(self) -> Result<EsiConfig, EsiError> {
        // Ensure background refresh percentage is set properly
        if !(self.jwk_background_refresh_threshold > 0) {
            return Err(EsiError::OAuthConfigError(
                OAuthConfigError::InvalidBackgroundRefreshThreshold,
            ));
        }
        if !(self.jwk_background_refresh_threshold < 100) {
            return Err(EsiError::OAuthConfigError(
                OAuthConfigError::InvalidBackgroundRefreshThreshold,
            ));
        }

        // Parse URLs
        let auth_url = match AuthUrl::new(self.auth_url.clone()) {
            Ok(url) => url,
            Err(_) => return Err(EsiError::OAuthConfigError(OAuthConfigError::InvalidAuthUrl)),
        };
        let token_url = match TokenUrl::new(self.token_url.clone()) {
            Ok(url) => url,
            Err(_) => {
                return Err(EsiError::OAuthConfigError(
                    OAuthConfigError::InvalidTokenUrl,
                ));
            }
        };

        Ok(EsiConfig {
            // URL Settings
            auth_url: auth_url,
            token_url: token_url,
            jwk_url: self.jwk_url,

            // JWT Key Settings
            jwk_cache_ttl: self.jwk_cache_ttl,
            jwk_refresh_max_retries: self.jwk_refresh_max_retries,
            jwk_refresh_backoff: self.jwk_refresh_backoff,
            jwk_refresh_timeout: self.jwk_refresh_timeout,
            jwk_refresh_cooldown: self.jwk_refresh_cooldown,

            // Background Refresh Settings
            jwk_background_refresh_enabled: self.jwk_background_refresh_enabled,
            jwk_background_refresh_threshold: self.jwk_background_refresh_threshold,
        })
    }

    /// Sets the EVE Online JWT key URL used to fetch keys to validate tokens.
    ///
    /// This method configures the JWT key URL for EVE Online OAuth2 to a custom URL.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `jwk_url` (&[`str`]): The EVE Online JWK URL.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with updated EVE Online JWK URL configuration.
    pub fn jwk_url(mut self, jwk_url: &str) -> Self {
        self.jwk_url = jwk_url.to_string();
        self
    }

    /// Sets the EVE Online OAuth2 authorizion URL
    ///
    /// This method configures the authorize URL for EVE Online oauth2.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `auth_url` (&[`str`]): The EVE Online oauth2 authorize URL.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with updated EVE Online OAuth2 authorization URL.
    pub fn auth_url(mut self, auth_url: &str) -> Self {
        self.auth_url = auth_url.to_string();
        self
    }

    /// Sets the EVE Online OAuth2 token URL
    ///
    /// This method configures the token URL for EVE Online oauth2 to a custom URL.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `token_url` (&[`str`]): The EVE Online oauth2 token URL.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with updated EVE Online OAuth2 token URL.
    pub fn token_url(mut self, token_url: &str) -> Self {
        self.token_url = token_url.to_string();
        self
    }

    /// Modifies the default lifetime of the JWT keys stored in cache
    ///
    /// By default, JWT keys are stored in cache for 3600 seconds (1 hour)
    /// before they are considered expired and need to be refreshed.
    ///
    /// Additionally, JWT keys are proactively refreshed by a background
    /// task at 80% expiration. You may wish to modify it with the
    /// [`Self::jwk_background_refresh_threshold`] method or disable the
    /// background refresh altogether with [`Self::jwk_background_refresh_enabled`]
    /// method.
    ///
    /// # Arguments
    /// - `seconds` ([`u64`]): The lifetime in seconds of the JWT keys stored in the cache.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with the updated JWT key cache TTL
    pub fn jwk_cache_ttl(mut self, seconds: u64) -> Self {
        self.jwk_cache_ttl = seconds;
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
    /// This does not affect the background JWT key refresh as it only makes one attempt with
    /// a 60 second cooldown between each attempt which can be modified with the
    /// [`Self::jwk_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `retry_attempts` ([`u64`]): The amount of retry attempts if a JWT key fetch fails.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with the updated JWK refresh max retries
    pub fn jwk_refresh_max_retries(mut self, retry_attempts: u64) -> Self {
        self.jwk_refresh_max_retries = retry_attempts;
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
    /// This does not affect the background JWT key refresh as it only makes one attempt with
    /// a 60 second cooldown between each attempt which can be modified with the
    /// [`Self::jwk_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `backoff_milliseconds` ([`u64`]): The exponential backoff duration in milliseconds between each attempt.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with the updated exponential backoff
    pub fn jwk_refresh_backoff(mut self, backoff_milliseconds: u64) -> Self {
        self.jwk_refresh_backoff = backoff_milliseconds;
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
    /// - [`EsiConfig`]: Instance with the modified timeout setting.
    pub fn jwk_refresh_timeout(mut self, timeout_seconds: u64) -> Self {
        self.jwk_refresh_timeout = timeout_seconds;
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
    /// - [`EsiConfig`]: Instance with the modified background refresh cooldown.
    pub fn jwk_refresh_cooldown(mut self, cooldown_seconds: u64) -> Self {
        self.jwk_refresh_cooldown = cooldown_seconds;
        self
    }

    /// Modifies whether or not the proactive background refresh when JWT keys are almost expired is enabled
    ///
    /// By default, when the JWT key cache is nearing expiration at around 80%, a background refresh task
    /// will be spawned to proactively refresh the keys. This behavior is thread safe and a more detailed
    /// description of how it works can be found at [`crate::oauth2::jwk::JwkApi::get_jwt_keys`].
    ///
    /// This functionality has been built with high volume applications in mind and will work for the
    /// vast majority of production use cases. In the instance where you do want to have more control
    /// over proactive JWT key refreshes consider disabling this and using a cron task to perform
    /// a refresh instead with the [`crate::oauth2::jwk::JwkApi::fetch_and_update_cache`] method which will
    /// update the cache regardless of expiration status.
    ///
    /// You can modify the % at which the proactive background refresh is triggered with the
    /// [`Self::jwk_background_refresh_threshold`] method.
    ///
    /// # Arguments
    /// - `background_refresh_enabled` ([`bool`]): A bool indicating whether or not the background refresh
    ///   is enabled.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with the modified status of whether or not the background refresh is enabled.
    pub fn jwk_background_refresh_enabled(mut self, background_refresh_enabled: bool) -> Self {
        self.jwk_background_refresh_enabled = background_refresh_enabled;
        self
    }

    /// The % of JWT key cache lifetime for when the proactive background JWT key cache refresh is triggered
    ///
    /// By default, when the JWT key cache reaches 80% of the default 3600 second cache lifetime, a proactive
    /// JWT key cache refresh will be triggered next time the [`crate::oauth2::jwk::JwkApi::get_jwt_keys`]
    /// method is called.
    ///
    /// Should the attempt fail, there will be a 60 second cooldown between attempts which can be modified with
    /// the [`Self::jwk_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `threshold_percent` ([`u64`]): A number representing the percentage of when the refresh should be triggered.
    ///
    /// # Returns
    /// - [`EsiConfig`]: Instance with the modified proactive background refresh threshold percentage.
    pub fn jwk_background_refresh_threshold(mut self, threshold_percentage: u64) -> Self {
        self.jwk_background_refresh_threshold = threshold_percentage;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures that all setter methods for [`EsiConfigBuilder`] work as expected
    ///
    /// Test Setup
    /// - Create a new instance of [`EsiConfigBuilder`] and use each setter method
    /// - Build the [`EsiConfigBuilder`] returning an [`EsiConfig`]
    ///
    /// Assertions
    /// - Assert URL settings were set as expected
    /// - Assert JWT key settings were set as expected
    /// - Assert JWT key background refresh settings were set as expected
    #[test]
    fn test_config_setter_methods() {
        let config = EsiConfig::builder()
            // URL settings
            .auth_url("https://example.com")
            .token_url("https://example.com")
            .jwk_url("https://example.com")
            // JWT key settings
            .jwk_cache_ttl(0)
            .jwk_refresh_max_retries(0)
            .jwk_refresh_backoff(0)
            .jwk_refresh_timeout(0)
            .jwk_refresh_cooldown(0)
            // Background refresh settings
            .jwk_background_refresh_enabled(false)
            .jwk_background_refresh_threshold(1)
            .build()
            .expect("Failed to build EsiConfig");

        // Assert URL settings were set
        let auth_url = AuthUrl::new("https://example.com".to_string()).unwrap();
        let token_url = TokenUrl::new("https://example.com".to_string()).unwrap();

        assert_eq!(config.auth_url, auth_url);
        assert_eq!(config.token_url, token_url);
        assert_eq!(config.jwk_url, "https://example.com");

        // Assert JWT key settings were set
        assert_eq!(config.jwk_cache_ttl, 0);
        assert_eq!(config.jwk_refresh_max_retries, 0);
        assert_eq!(config.jwk_refresh_backoff, 0);
        assert_eq!(config.jwk_refresh_timeout, 0);
        assert_eq!(config.jwk_refresh_cooldown, 0);

        // Assert JWT key background refresh settings were set
        assert_eq!(config.jwk_background_refresh_enabled, false);
        assert_eq!(config.jwk_background_refresh_threshold, 1);
    }

    /// Expect an error setting the JWK background refresh threshold to 0
    ///
    /// # Test Setup
    /// - Attempt to build a [`EsiConfig`] with the jwk_background_refresh_threshold to 0
    ///
    /// # Assertions
    /// - Assert result is an error
    /// - Assert error is of type [`OAuthConfigError::InvalidBackgroundRefreshThreshold`]
    #[test]
    fn test_invalid_background_refresh_threshold_0() {
        // Create a EsiConfig with invalid threshold percent
        let result = EsiConfig::builder()
            .jwk_background_refresh_threshold(0)
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

    /// Expect an error setting the JWK background refresh threshold to 100
    ///
    /// # Test Setup
    /// - Attempt to build an [`EsiConfig`] with the jwk_background_refresh_threshold to 100
    ///
    /// # Assertions
    /// - Assert result is an error
    /// - Assert error is of type [`OAuthConfigError::InvalidBackgroundRefreshThreshold`]
    #[test]
    fn test_invalid_background_refresh_threshold_100() {
        // Create a EsiConfig with invalid threshold percent
        let result = EsiConfig::builder()
            .jwk_background_refresh_threshold(100)
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
