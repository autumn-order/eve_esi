//! # EVE Online ESI Client Config
//!
//! Provides methods to override defaults for the [`Client`](crate::Client). This allows the
//! modification of the base ESI URL, OAuth2 endpoint URLs and the logic of how JWT
//! key caching and refreshing is handled.
//!
//! ## Features
//! - Override the base ESI URL
//! - Override EVE Online OAuth2 authorization, JWT key, and token endpoint URLs
//! - Adjust expiration time & threshold for a proactive refresh for the JWT key cache used to validate tokens
//! - Adjust the timeout between sets of JWT key refresh attempts
//! - Adjust backoff period (wait time) beteween attempts and how many retries should be made to refresh JWT keys
//! - Enable/disable the proactive background JWT key refresh
//!
//! ## Builder Methods
//!
//! | Method          | Purpose                                    |
//! | --------------- | ------------------------------------------ |
//! | `new`           | Create a new [`ConfigBuilder`]          |
//! | `build`         | Build the [`Config`]                    |
//! | `esi_url`       | Base URL for ESI endpoints                 |
//! | `auth_url`      | URL for sign in with EVE Online            |
//! | `token_url`     | URL to retrieve access tokens for OAuth2   |
//! | `jwk_url`       | URL for JWT keys to validate tokens        |
//! | `jwk_cache_ttl`     | The time that JWT keys are cached for      |
//! | `jwk_refresh_backoff`     | How long to wait between retries         |
//! | `jwk_refresh_timeout`     | How long to wait for another thread to refresh |
//! | `jwk_refresh_cooldown`    | Cooldown between sets of JWT key refresh attempts |
//! | `jwk_refresh_max_retries` | Amount of retries when a key fetch fails |
//! | `jwk_background_refresh_enabled` | Enable/disable background refresh          |
//! | `jwk_background_refresh_threshold` | Percentage at which cache is refreshed proactively |
//!
//! ## Usage
//!
//! ```
//! use std::time::Duration;
//!
//! // Build a config to override defaults
//! let config = eve_esi::Config::builder()
//!     // Set JWT key cache lifetime to 7200 seconds representing 2 hours
//!     .jwk_cache_ttl(Duration::from_secs(7200))
//!     .build()
//!     .expect("Failed to build ESI Config");
//!
//! // Apply config settings to Client
//! let esi_client = eve_esi::Client::builder()
//!     .config(config)
//!     .user_agent("MyApp/1.0 (contact@example.com")
//!     .build()
//!     .expect("Failed to build ESI Client");
//! ```

use std::time::Duration;

use oauth2::{AuthUrl, TokenUrl};

use crate::{
    constant::{DEFAULT_AUTH_URL, DEFAULT_ESI_URL, DEFAULT_TOKEN_URL},
    error::{ConfigError, Error},
    oauth2::jwk::cache::JwtKeyCacheConfig,
};

/// Configuration settings for the [`Client`](crate::Client)
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct Config {
    // URL settings
    /// The base EVE Online ESI API URL
    pub(crate) esi_url: String,
    /// Authorization URL used to login with EVE Online's OAuth2
    pub(crate) auth_url: AuthUrl,
    /// Token URL which provides an access token for authenticated ESI endpoints
    pub(crate) token_url: TokenUrl,

    // JWT Key Settings
    /// Config for JWT key caching & refreshing
    pub(crate) jwt_key_cache_config: JwtKeyCacheConfig,
}

/// Builder struct for configuring & constructing an [`Config`] to override default [`Client`](crate::Client) settings
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct ConfigBuilder {
    // URL settings
    /// The base EVE Online ESI API URL
    pub(crate) esi_url: String,
    /// Authorization URL used to login with EVE Online's OAuth2
    pub(crate) auth_url: String,
    /// Token URL which provides an access token for authenticated ESI endpoints
    pub(crate) token_url: String,

    // OAuth2 JWT key config
    /// Config for OAuth2 JWT key caching & refreshing
    pub(crate) jwt_key_cache_config: JwtKeyCacheConfig,
}

impl Config {
    /// Creates a new instance of [`Config`] with default settings
    ///
    /// For details see [module-level documentation](self).
    ///
    /// # Returns
    /// - [`Config`]: With the default configuration
    ///
    /// # Errors
    /// - [`Error`]: If the default [`ConfigBuilder::jwk_background_refresh_threshold`] is configured incorrectly.
    pub fn new() -> Result<Self, Error> {
        ConfigBuilder::new().build()
    }

    /// Returns a [`ConfigBuilder`] instance used to configure JWT key related settings
    ///
    /// Allows for the configuration of the [`Config`] using the [`ConfigBuilder`]
    /// setter methods to override the default configuration.
    ///
    /// For details see [module-level documentation](self).
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the default config which can be overridden with setter methods.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

impl ConfigBuilder {
    /// Creates a new [`ConfigBuilder`] instance used to build an [`Config`]
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the default settings that can be modified with the setter methods.
    pub fn new() -> Self {
        Self {
            // URL settings
            esi_url: DEFAULT_ESI_URL.to_string(),
            auth_url: DEFAULT_AUTH_URL.to_string(),
            token_url: DEFAULT_TOKEN_URL.to_string(),

            // OAuth2 JWT key config
            jwt_key_cache_config: JwtKeyCacheConfig::new(),
        }
    }

    /// Builds a [`Config`] instance
    ///
    /// Converts an [`ConfigBuilder`] into a [`Config`] with the configured values that
    /// were set with the builder methods.
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`Config`]: instance with the settings configured on the builder
    ///
    /// # Errors
    /// Returns an [`Error`] if one of the following occurs:
    /// - The [`Self::jwk_background_refresh_threshold`] method is given a value less than 1 or over 99
    /// - The [`Self::auth_url`] method is given an invalid URL
    /// - The [`Self::token_url`] method is given an invalid URL
    pub fn build(self) -> Result<Config, Error> {
        // Ensure background refresh percentage is set properly
        if !(self.jwt_key_cache_config.background_refresh_threshold > 0) {
            return Err(Error::ConfigError(
                ConfigError::InvalidBackgroundRefreshThreshold,
            ));
        }
        if !(self.jwt_key_cache_config.background_refresh_threshold < 100) {
            return Err(Error::ConfigError(
                ConfigError::InvalidBackgroundRefreshThreshold,
            ));
        }

        // Parse URLs
        let auth_url = match AuthUrl::new(self.auth_url.clone()) {
            Ok(url) => url,
            Err(_) => return Err(Error::ConfigError(ConfigError::InvalidAuthUrl)),
        };
        let token_url = match TokenUrl::new(self.token_url.clone()) {
            Ok(url) => url,
            Err(_) => {
                return Err(Error::ConfigError(ConfigError::InvalidTokenUrl));
            }
        };

        Ok(Config {
            // URL settings
            esi_url: self.esi_url,
            auth_url: auth_url,
            token_url: token_url,

            // JWT key cache settings
            jwt_key_cache_config: self.jwt_key_cache_config,
        })
    }

    /// Sets the EVE Online ESI base URL
    ///
    /// This method configures the base URL for EVE Online ESI.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `esi_url` (&[`str`]): The EVE Online ESI API base URL.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the updated ESI URL
    pub fn esi_url(mut self, esi_url: &str) -> Self {
        self.esi_url = esi_url.to_string();
        self
    }

    /// Sets the EVE Online OAuth2 authorizion URL
    ///
    /// This method configures the authorize URL for EVE Online oauth2.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `auth_url` (&[`str`]): The EVE Online OAuth2 authorization endpoint URL.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with updated EVE Online OAuth2 authorization URL.
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
    /// - `token_url` (&[`str`]): The EVE Online OAuth2 token endpoint URL.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with updated EVE Online OAuth2 token URL.
    pub fn token_url(mut self, token_url: &str) -> Self {
        self.token_url = token_url.to_string();
        self
    }

    /// Sets the EVE Online JWT key URL used to fetch keys to validate tokens.
    ///
    /// This method configures the JWT key URL for EVE Online OAuth2 to a custom URL.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `jwk_url` (&[`str`]): The EVE Online JWK endpoint URL.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with updated EVE Online JWK URL configuration.
    pub fn jwk_url(mut self, jwk_url: &str) -> Self {
        self.jwt_key_cache_config.jwk_url = jwk_url.to_string();
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
    /// - `duration` ([`Duration`]): The lifetime of the JWT keys stored in the cache.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the updated JWT key cache TTL
    pub fn jwk_cache_ttl(mut self, duration: Duration) -> Self {
        self.jwt_key_cache_config.cache_ttl = duration;
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
    /// - `duration` ([`Duration`]): The exponential backoff duration between each attempt.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the updated exponential backoff
    pub fn jwk_refresh_backoff(mut self, duration: Duration) -> Self {
        self.jwt_key_cache_config.refresh_backoff = duration;
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
    /// - `duration` ([`Duration`]): Timeout duration to wait for another thread to complete a
    ///   JWT key refresh.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the modified timeout setting.
    pub fn jwk_refresh_timeout(mut self, duration: Duration) -> Self {
        self.jwt_key_cache_config.refresh_timeout = duration;
        self
    }

    /// Modifies the cooldown between sets of JWT key cache refresh attempts in the event of failure
    ///
    /// By default, when a set of JWT key cache refresh attempts fail there will be a cooldown of 60 seconds
    /// between the next set of attempts to refresh JWT keys before expiration.
    ///
    /// # Arguments
    /// - `duration` ([`Duration`]): Cooldown duration between background JWT key cache refresh attempts.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the modified background refresh cooldown.
    pub fn jwk_refresh_cooldown(mut self, duration: Duration) -> Self {
        self.jwt_key_cache_config.refresh_cooldown = duration;
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
    /// - `retry_attempts` ([`u32`]): The amount of retry attempts if a JWT key fetch fails.
    ///
    /// # Returns
    /// - [`ConfigBuilder`]: Instance with the updated JWK refresh max retries
    pub fn jwk_refresh_max_retries(mut self, retry_attempts: u32) -> Self {
        self.jwt_key_cache_config.refresh_max_retries = retry_attempts;
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
    /// - [`ConfigBuilder`]: Instance with the background refresh is enabled or disabled.
    pub fn jwk_background_refresh_enabled(mut self, background_refresh_enabled: bool) -> Self {
        self.jwt_key_cache_config.background_refresh_enabled = background_refresh_enabled;
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
    /// - [`ConfigBuilder`]: Instance with the modified proactive background refresh threshold percentage.
    pub fn jwk_background_refresh_threshold(mut self, threshold_percentage: u64) -> Self {
        self.jwt_key_cache_config.background_refresh_threshold = threshold_percentage;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures that all setter methods for [`ConfigBuilder`] work as expected
    ///
    /// Test Setup
    /// - Create a new instance of [`ConfigBuilder`] and use each setter method
    /// - Build the [`ConfigBuilder`] returning an [`Config`]
    ///
    /// Assertions
    /// - Assert URL settings were set as expected
    /// - Assert JWT key settings were set as expected
    /// - Assert JWT key background refresh settings were set as expected
    #[test]
    fn test_config_setter_methods() {
        let zero_seconds = Duration::from_secs(0);

        let config = Config::builder()
            // URL settings
            .auth_url("https://example.com")
            .token_url("https://example.com")
            .jwk_url("https://example.com")
            // JWT key settings
            .jwk_cache_ttl(zero_seconds)
            .jwk_refresh_backoff(zero_seconds)
            .jwk_refresh_timeout(zero_seconds)
            .jwk_refresh_cooldown(zero_seconds)
            .jwk_refresh_max_retries(0)
            // Background refresh settings
            .jwk_background_refresh_enabled(false)
            .jwk_background_refresh_threshold(1)
            .build()
            .expect("Failed to build Config");

        // Assert URL settings were set
        let auth_url = AuthUrl::new("https://example.com".to_string()).unwrap();
        let token_url = TokenUrl::new("https://example.com".to_string()).unwrap();

        assert_eq!(config.auth_url, auth_url);
        assert_eq!(config.token_url, token_url);
        assert_eq!(config.jwt_key_cache_config.jwk_url, "https://example.com");

        // Assert JWT key settings were set
        assert_eq!(config.jwt_key_cache_config.cache_ttl, zero_seconds);
        assert_eq!(config.jwt_key_cache_config.refresh_backoff, zero_seconds);
        assert_eq!(config.jwt_key_cache_config.refresh_timeout, zero_seconds);
        assert_eq!(config.jwt_key_cache_config.refresh_cooldown, zero_seconds);
        assert_eq!(config.jwt_key_cache_config.refresh_max_retries, 0);

        // Assert JWT key background refresh settings were set
        assert_eq!(
            config.jwt_key_cache_config.background_refresh_enabled,
            false
        );
        assert_eq!(config.jwt_key_cache_config.background_refresh_threshold, 1);
    }

    /// Expect an error setting the JWK background refresh threshold to 0
    ///
    /// # Test Setup
    /// - Attempt to build a [`Config`] with the jwk_background_refresh_threshold to 0
    ///
    /// # Assertions
    /// - Assert result is an error
    /// - Assert error is of type [`OAuthConfigError::InvalidBackgroundRefreshThreshold`]
    #[test]
    fn test_invalid_background_refresh_threshold_0() {
        // Create a Config with invalid threshold percent
        let result = Config::builder()
            .jwk_background_refresh_threshold(0)
            .build();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type OAuthConfigError::InvalidBackgroundRefreshThreshold
        assert!(matches!(
            result,
            Err(Error::ConfigError(
                ConfigError::InvalidBackgroundRefreshThreshold
            ))
        ))
    }

    /// Expect an error setting the JWK background refresh threshold to 100
    ///
    /// # Test Setup
    /// - Attempt to build an [`Config`] with the jwk_background_refresh_threshold to 100
    ///
    /// # Assertions
    /// - Assert result is an error
    /// - Assert error is of type [`ConfigError::InvalidBackgroundRefreshThreshold`]
    #[test]
    fn test_invalid_background_refresh_threshold_100() {
        // Create a Config with invalid threshold percent
        let result = Config::builder()
            .jwk_background_refresh_threshold(100)
            .build();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type ConfigError::InvalidBackgroundRefreshThreshold
        assert!(matches!(
            result,
            Err(Error::ConfigError(
                ConfigError::InvalidBackgroundRefreshThreshold
            ))
        ))
    }

    /// Tests the attempting initialize an Config with an invalid auth_url
    ///
    /// # Test Setup
    /// - Attempt to build an Config with the auth_url set to an invalid URL.
    ///
    /// # Assertions
    /// - Verifies that the error response is ConfigError::InvalidAuthUrl
    #[test]
    fn test_invalid_auth_url() {
        // Create an Config with an invalid auth_url
        let result = Config::builder().auth_url("invalid_url").build();

        // Assert result is an Error
        assert!(result.is_err());

        match result {
            // Assert error is of the ConfigError:InvalidAuthUrl variant
            Err(Error::ConfigError(ConfigError::InvalidAuthUrl)) => {}
            _ => panic!("Expected InvalidAuthUrl error"),
        }
    }

    /// Tests the attempting initialize an Config with an invalid token_url
    ///
    /// # Test Setup
    /// - Attempt to build an Config with the token_url set to an invalid URL.
    ///
    /// # Assertions
    /// - Verifies that the error response is ConfigError::InvalidTokenUrl
    #[test]
    fn test_invalid_token_url() {
        // Create an Config with an invalid token_url
        let result = Config::builder().token_url("invalid_url").build();

        // Assert result is an Error
        assert!(result.is_err());

        match result {
            // Assert error is of the ConfigError:InvalidTokenUrl variant
            Err(Error::ConfigError(ConfigError::InvalidTokenUrl)) => {}
            _ => panic!("Expected InvalidTokenUrl error"),
        }
    }
}
