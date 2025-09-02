//! # EVE Online OAuth2 Config
//!
//! This module provides the [`OAuth2Config`] struct for overriding default OAuth2 settings for
//! the [`crate::EsiClient`].
//!
//! Generally the default settings will work perfectly fine for a production application.
//! This is most useful for overriding the default EVE OAuth2 API URLs for the purposes of writing tests
//! with crates such as [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
//!
//! - See [module-level] documentation for a higher level overview and usage example
//! - See the [builder](crate::builder) module for configuration options & details.
//!
//! ## Config Creation
//! The config is created using the builder pattern: [`OAuth2Config::builder`];

use oauth2::{AuthUrl, TokenUrl};

use crate::error::EsiError;

use super::builder::OAuth2ConfigBuilder;

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
    ///
    /// Uses the AuthUrl type from the [`oauth2`] crate in order to return
    /// an error when building the config instead of during runtime
    /// if the URL is incorrectly formatted.
    pub(crate) auth_url: AuthUrl,
    /// Token URL endpoint used to retrieve tokens to authenticate users
    ///
    /// Uses the TokenUrl type from the [`oauth2`] crate in order to return
    /// an error when building the config instead of during runtime
    /// if the URL is incorrectly formatted.
    pub(crate) token_url: TokenUrl,
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
    /// Cooldown period in seconds after a failed set of JWT key refresh attempts (default 60 seconds)
    pub(crate) jwk_refresh_cooldown: u64,

    // JWT key cache background refresh settings
    /// Determines whether or not a background task is spawned to refresh JWT keys nearing expiration proactively
    pub(crate) jwk_background_refresh_enabled: bool,
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
    pub fn default() -> Result<OAuth2Config, EsiError> {
        OAuth2ConfigBuilder::new().build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::constant::{
        DEFAULT_AUTH_URL, DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT, DEFAULT_JWK_CACHE_TTL,
        DEFAULT_JWK_REFRESH_BACKOFF, DEFAULT_JWK_REFRESH_COOLDOWN, DEFAULT_JWK_REFRESH_MAX_RETRIES,
        DEFAULT_JWK_REFRESH_TIMEOUT, DEFAULT_JWK_URL, DEFAULT_TOKEN_URL,
    };

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
        let config = OAuth2Config::default().expect("Failed to build OAuth2Config");

        // Assert URLs are expected defaults
        let auth_url = AuthUrl::new(DEFAULT_AUTH_URL.to_string()).unwrap();
        let token_url = TokenUrl::new(DEFAULT_TOKEN_URL.to_string()).unwrap();

        assert_eq!(config.auth_url, auth_url);
        assert_eq!(config.token_url, token_url);
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
        assert_eq!(config.jwk_refresh_cooldown, DEFAULT_JWK_REFRESH_COOLDOWN);
        assert_eq!(
            config.jwk_background_refresh_threshold_percent,
            DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT
        );
    }
}
