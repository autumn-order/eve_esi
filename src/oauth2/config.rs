//! # EVE Online ESI OAuth2 Config
//!
//! Provides methods to modify the default settings for the eve_esi crate regarding the
//! EVE OAuth2 API endpoint URLs or the logic of how JWT (JSON web token) key caching and refreshing is
//! handled which are used to validate tokens for user authentication.
//!
//! Generally the default variables will work perfectly fine for a production application.
//! This is most useful for overriding the default EVE OAuth2 API URLs for the purposes of writing tests
//! with crates such as [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
//!
//! ## Features
//! - Override default authentication, JWT token, JWT token key endpoint URLs
//! - Adjust expiration times for JWT key cache
//! - Adjust the timeout for waiting for JWT key refreshes
//! - Adjust wait time/backoff period beteween refresh and how many retries should be made to fetch JWT keys
//! - Enable/disable the practive background JWT key refresh
//!
//! ## Config Creation
//! The config is created using the builder pattern: [`OAuth2Config::builder`];
//!
//! ## Builder Methods
//!
//! | Method          | Purpose                                    |
//! | --------------- | ------------------------------------------ |
//! | `new`           | Create a new OAuth2Config builder          |
//! | `build`         | Build the OAuth2 config                    |
//! | `auth_url`      | URL for EVE OAuth2 authentication          |
//! | `token_url`     | URL for EVE OAuth2 token                   |
//! | `jwk_url`       | URL for EVE OAuth2 token keys              |
//! | `jwk_cache_ttl` | The time that JWT keys are cached for      |
//! | `jwk_refresh_max_retries` | Amount of retries when a key fetch fails |
//! | `jwk_refresh_backoff`     | How long to wait between retries |
//! | `jwk_refresh_timeout`     | How long to wait for another thread to refresh |
//! | `jwk_background_refresh_enabled` | Enable/disable background refresh |
//! | `jwk_background_refresh_cooldown` | How long to wait between background refresh attempts |
//! | `jwk_background_refresh_threshold_percent` | Percentage at which cache is refreshed proactively |
//!
//! ## Usage
//!
//! ```
//! use eve_esi::EsiClient;
//! use eve_esi::oauth2::OAuth2Config;
//!
//! // Set 2 hour JWT key cache lifetime in seconds
//! let config = OAuth2Config::builder()
//!     .jwk_cache_ttl(7200).build();
//!
//! // Apply config to EsiClient
//! let esi_client = EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .oauth2_config(config)
//!     .build()
//!     .expect("Failed to build EsiClient");
//! ```

use crate::constant::{
    DEFAULT_AUTH_URL, DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN,
    DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT, DEFAULT_JWK_CACHE_TTL,
    DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_MAX_RETRIES, DEFAULT_JWK_REFRESH_TIMEOUT,
    DEFAULT_JWK_URL, DEFAULT_TOKEN_URL,
};

/// Configuration for modifying OAuth2 related settings for the EsiClient
///
/// This is used for overriding the EVE OAuth2 API endpoint URLs
/// for testing purposes or for more precise control over how the JWT keys
/// used to validate tokens are cached and fetched.
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct OAuth2Config {
    // EVE OAuth2 API URL overrides
    /// Authentication URL endpoint for the EVE Online OAuth2 login flow
    pub(crate) auth_url: String,
    /// Token URL endpoint used to retrieve tokens to authenticate users
    pub(crate) token_url: String,
    /// JSON web token key URL that provides keys used to validate tokens
    pub(crate) jwk_url: String,

    // JWT key cache settings
    /// JWT key cache lifetime before expiration in seconds (3600 seconds representing 1 hour)
    pub(crate) jwk_cache_ttl: u64,
    /// Maximum number of retries for JWT key refresh when cache is empty or expired (default 2 retries)
    pub(crate) jwk_refresh_max_retries: u64,
    /// Backoff period in seconds after a JWT key refresh failure when cache is empty or expired (default 100 milliseconds)
    pub(crate) jwk_refresh_backoff: u64,
    /// Timeout in seconds when waiting for another thread to refresh JWT key (default 5 seconds)
    pub(crate) jwk_refresh_timeout: u64,

    // JWT key cache background refresh settings
    /// Determines whether or not a background task is spawned to refresh JWT keys nearing expiration proactively
    pub(crate) jwk_background_refresh_enabled: bool,
    /// Cooldown period in seconds after a JWT key refresh failure (default 60 seconds)
    pub(crate) jwk_background_refresh_cooldown: u64,
    /// Percentage of jwk_cache_ttl for when the background JWT key refresh is triggered (default 80%)
    pub(crate) jwk_background_refresh_threshold_percent: u64,
}

/// Builder struct for configuring & constructing an [`OAuth2Config`]
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct OAuth2ConfigBuilder {
    // EVE OAuth2 API URL overrides
    /// Authentication URL endpoint for the EVE Online OAuth2 login flow
    pub(crate) auth_url: String,
    /// Token URL endpoint used to retrieve tokens to authenticate users
    pub(crate) token_url: String,
    /// JSON web token key URL that provides keys used to validate tokens
    pub(crate) jwk_url: String,

    // JWT key cache settings
    /// JWT key cache lifetime before expiration in seconds (default 1 hour)
    pub(crate) jwk_cache_ttl: u64,
    /// Maximum number of retries for JWT key refresh when cache is empty or expired (default 2 retries)
    pub(crate) jwk_refresh_max_retries: u64,
    /// Backoff period in seconds after a JWT key refresh failure when cache is empty or expired (default 100 milliseconds)
    pub(crate) jwk_refresh_backoff: u64,
    /// Timeout in seconds when waiting for another thread to refresh JWT key (default 5 seconds)
    pub(crate) jwk_refresh_timeout: u64,

    // JWT key cache background refresh settings
    /// Determines whether or not a background task is spawned to refresh JWT keys nearing expiration proactively
    pub(crate) jwk_background_refresh_enabled: bool,
    /// Cooldown period in seconds after a JWT key refresh failure (default 60 seconds)
    pub(crate) jwk_background_refresh_cooldown: u64,
    /// Percentage of jwk_cache_ttl for when the background JWT key refresh is triggered (default 80%)
    pub(crate) jwk_background_refresh_threshold_percent: u64,
}

impl OAuth2Config {
    /// Returns an [`OAuth2ConfigBuilder`] instance used to configure OAuth2 related settings
    ///
    /// Allows for the configuration of the [`OAuth2Config`] using the [`OAuth2ConfigBuilder`] methods
    /// to change the default configuration to custom values using the setter methods.
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`OAuth2ConfigBuilder`]: Instance with the default settings that can be overridden with setter methods.
    pub fn builder() -> OAuth2ConfigBuilder {
        OAuth2ConfigBuilder::new()
    }

    /// Creates a new [`OAuth2Config`] with only the default values
    ///
    /// Use the [`Self::builder`] method to create an [`OAuth2ConfigBuilder`] instead that can
    /// be used to override the default values with setter methods.
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance configured with the default values
    pub fn default() -> OAuth2Config {
        OAuth2ConfigBuilder::new().build()
    }
}

impl OAuth2ConfigBuilder {
    /// Creates a new [`OAuth2ConfigBuilder`] instance used to build an [`OAuth2Config`]
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`OAuth2ConfigBuilder`]: Instance with the default settings that can be overridden with setter methods.
    pub fn new() -> Self {
        Self {
            // EVE OAuth2 API URL overrides
            auth_url: DEFAULT_AUTH_URL.to_string(),
            token_url: DEFAULT_TOKEN_URL.to_string(),
            jwk_url: DEFAULT_JWK_URL.to_string(),

            // JWT key cache settings
            jwk_cache_ttl: DEFAULT_JWK_CACHE_TTL,
            jwk_refresh_max_retries: DEFAULT_JWK_REFRESH_MAX_RETRIES,
            jwk_refresh_backoff: DEFAULT_JWK_REFRESH_BACKOFF,
            jwk_refresh_timeout: DEFAULT_JWK_REFRESH_TIMEOUT,

            // JWT key cache background refresh settings
            jwk_background_refresh_enabled: true,
            jwk_background_refresh_cooldown: DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN,
            jwk_background_refresh_threshold_percent:
                DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT,
        }
    }

    /// Builds an [`OAuth2Config`] instance
    ///
    /// Converts an [`OAuth2ConfigBuilder`] into an [`OAuth2`] config
    /// with the configured values that were set with the builder methods.
    ///
    /// For a full overview, features, and usage example, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`OAuth2Config`]: instance with the settings configured on the builder
    pub fn build(self) -> OAuth2Config {
        OAuth2Config {
            // EVE OAuth2 API URL overrides
            auth_url: self.auth_url,
            token_url: self.token_url,
            jwk_url: self.jwk_url,

            // JWT key cache settings
            jwk_cache_ttl: self.jwk_cache_ttl,
            jwk_refresh_max_retries: self.jwk_refresh_max_retries,
            jwk_refresh_backoff: self.jwk_refresh_backoff,
            jwk_refresh_timeout: self.jwk_refresh_timeout,

            // JWT key cache background refresh settings
            jwk_background_refresh_enabled: self.jwk_background_refresh_enabled,
            jwk_background_refresh_cooldown: self.jwk_background_refresh_cooldown,
            jwk_background_refresh_threshold_percent: self.jwk_background_refresh_threshold_percent,
        }
    }

    /// Sets the EVE Online oauth2 authorize URL to a custom URL.
    ///
    /// This method configures the authorize URL for EVE Online oauth2.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `auth_url` (&[`str`]): The EVE Online oauth2 authorize URL.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with updated EVE Online oauth2 authorize URL configuration.
    pub fn auth_url(mut self, auth_url: &str) -> Self {
        self.auth_url = auth_url.to_string();
        self
    }

    /// Sets the EVE Online oauth2 token URL to a custom URL.
    ///
    /// This method configures the token URL for EVE Online oauth2 to a custom URL.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `token_url` (&[`str`]): The EVE Online oauth2 token URL.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with updated EVE Online oauth2 token URL configuration.
    pub fn token_url(mut self, token_url: &str) -> Self {
        self.token_url = token_url.to_string();
        self
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
    /// This does not affect the background JWT key refresh which is instead modified
    /// with the [`Self::jwk_background_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `retry_attempts` ([`u64`]): The amount of retry attempts if a JWT key fetch fails.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the updated JWK refresh max retries
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
    /// This does not affect the background JWT key refresh which is instead modified
    /// with the [`Self::jwk_background_refresh_cooldown`] method.
    ///
    /// # Arguments
    /// - `backoff_milliseconds` ([`u64`]): The exponential backoff duration in milliseconds between each attempt.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the updated exponential backoff
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
    /// - [`OAuth2Config`]: Instance with the modified timeout setting.
    pub fn jwk_refresh_timeout(mut self, timeout_seconds: u64) -> Self {
        self.jwk_refresh_timeout = timeout_seconds;
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
    pub fn jwk_background_refresh_enabled(mut self, background_refresh_enabled: bool) -> Self {
        self.jwk_background_refresh_enabled = background_refresh_enabled;
        self
    }

    /// Modifies the cooldown between background JWT key cache refresh attempts should the attempt fail
    ///
    /// By default, when a background JWT key cache refresh attempt fails there will be a cooldown of 60 seconds
    /// between the next attempt to proactively refresh JWT keys before expiration.
    ///
    /// The proactive refresh is first triggered when 80% of the 3600 JWT key cache lifetime has elapsed,
    /// this can be modified with the [`Self::jwk_background_refresh_threshold_percent`] method.
    ///
    /// Around a dozen attempts will be made by default before the cache fully expires given the default settings.
    ///
    /// # Arguments
    /// - `cooldown_seconds` ([`u64`]): Cooldown in seconds between background JWT key cache refresh attempts.
    ///
    /// # Returns
    /// - [`OAuth2Config`]: Instance with the modified background refresh cooldown.
    pub fn jwk_background_refresh_cooldown(mut self, cooldown_seconds: u64) -> Self {
        self.jwk_background_refresh_cooldown = cooldown_seconds;
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
    pub fn jwk_background_refresh_threshold_percent(mut self, threshold_percentage: u64) -> Self {
        self.jwk_background_refresh_threshold_percent = threshold_percentage;
        self
    }
}
#[cfg(test)]
#[cfg(test)]
mod oauth2_config_tests {
    use super::*;

    /// Ensures that all defaults for the [`OAuth2Config::default`] method are set as expected
    ///
    /// Test Setup
    /// - Use the [`OAuth2Config::default`] method to create a config with default values
    ///
    /// Assertions
    /// - Assert all URLs are the expected defaults
    /// - Assert all JWT key cache settings are the expected defaults
    /// - Assert all JWT key cache background refresh settings are the expected defaults
    #[test]
    fn test_config_default_values() {
        let config = OAuth2Config::default();

        // Assert URLs are expected defaults
        assert_eq!(config.auth_url, DEFAULT_AUTH_URL);
        assert_eq!(config.token_url, DEFAULT_TOKEN_URL);
        assert_eq!(config.jwk_url, DEFAULT_JWK_URL);

        // Assert JWT key settings are expected defaults
        assert_eq!(config.jwk_cache_ttl, DEFAULT_JWK_CACHE_TTL);
        assert_eq!(
            config.jwk_refresh_max_retries,
            DEFAULT_JWK_REFRESH_MAX_RETRIES
        );
        assert_eq!(config.jwk_refresh_backoff, DEFAULT_JWK_REFRESH_BACKOFF);
        assert_eq!(config.jwk_refresh_timeout, DEFAULT_JWK_REFRESH_TIMEOUT);

        // Assert JWT key cache background refresh settings are expected defaults
        assert_eq!(config.jwk_background_refresh_enabled, true);
        assert_eq!(
            config.jwk_background_refresh_cooldown,
            DEFAULT_JWK_BACKGROUND_REFRESH_COOLDOWN
        );
        assert_eq!(
            config.jwk_background_refresh_threshold_percent,
            DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT
        );
    }
}

#[cfg(test)]
mod oauth2_config_builder_tests {
    use super::*;

    /// Ensures that all setter methods for [`OAuth2ConfigBuilder`] work as expected
    ///
    /// Test Setup
    /// - Create a new instance of [`OAuth2ConfigBuilder`] and modify each field
    /// - Build the [`OAuth2ConfigBuilder`] returning an [`OAuth2Config`]
    ///
    /// Assertions
    /// - Assert all URLs were set as expected
    /// - Assert all JWT key cache settings were set as expected
    /// - Assert all JWT key cache background refresh settings were set as expected
    #[test]
    fn test_config_setter_methods() {
        let config = OAuth2ConfigBuilder::new()
            // URL settings
            .auth_url("https://example.com")
            .token_url("https://example.com")
            .jwk_url("https://example.com")
            // JWT key cache settings
            .jwk_cache_ttl(0)
            .jwk_refresh_max_retries(0)
            .jwk_refresh_backoff(0)
            .jwk_refresh_timeout(0)
            // JWT key cache background refresh settings
            .jwk_background_refresh_enabled(false)
            .jwk_background_refresh_cooldown(0)
            .jwk_background_refresh_threshold_percent(0)
            .build();

        // Assert URLs were set
        assert_eq!(config.auth_url, "https://example.com");
        assert_eq!(config.token_url, "https://example.com");
        assert_eq!(config.jwk_url, "https://example.com");

        // Assert JWT key cache settings were set
        assert_eq!(config.jwk_cache_ttl, 0);
        assert_eq!(config.jwk_refresh_max_retries, 0);
        assert_eq!(config.jwk_refresh_backoff, 0);
        assert_eq!(config.jwk_refresh_timeout, 0);

        // Assert JWT key cache background refresh settings were set
        assert_eq!(config.jwk_background_refresh_enabled, false);
        assert_eq!(config.jwk_background_refresh_cooldown, 0);
        assert_eq!(config.jwk_background_refresh_threshold_percent, 0);
    }
}
