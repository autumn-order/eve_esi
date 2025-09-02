//! Errors related to the OAuth2 portion of the EVE ESI crate
//!
//! Provides an enum for runtime related errors, [`OAuthError`], and an enum
//! for OAuth2 configuration related errors, [`OAuthConfigError`]. Each
//! enum provides detailed error messages as well as instructions on how to
//! resolve the issues which could occur.
//!
//! # Error Types
//! - [`OAuthError`]: Runtime errors related to EVE OAuth2
//! - [`OAuthConfigError`]: Configuration errors related to EVE OAuth2
//!
//! # Example
//! ```
//! use eve_esi::error::{EsiError, OAuthError};
//! use eve_esi::oauth2::ScopeBuilder;
//!
//! let esi_client = eve_esi::EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     // Don't set .client_id()
//!     // Don't set .client_secret()
//!     // Don't set .callback_url()
//!     .build()
//!     .expect("Failed to build EsiClient");
//!
//! // Using OAuth2 without configuring required settings causes a runtime error.
//! let scopes = ScopeBuilder::new()
//!     .public_data()
//!     .build();
//! let result = esi_client.oauth2().initiate_oauth_login(scopes);
//!
//! assert!(matches!(result, Err(EsiError::OAuthError(OAuthError::OAuth2NotConfigured))));
//! ```

use oauth2::basic::BasicErrorResponseType;
use oauth2::{HttpClientError, RequestTokenError, StandardErrorResponse};
use thiserror::Error;

/// Runtime errors related to OAuth2 authentication for the EVE ESI client library.
///
/// This enum represents various runtime error conditions that may arise when attempting to
/// authenticate with EVE Online's OAuth2 process. It provides detailed error messages
/// with instructions on how to resolve each issue.
///
/// See the [module-level documentation](self) for an overview and usage example.
///
/// # Variants
/// - [`MissingClientId`](OAuthError::MissingClientId): The required client ID for EVE Online OAuth2 was not provided.
/// - [`MissingClientSecret`](OAuthError::MissingClientSecret): The required client secret for EVE Online OAuth2 was not provided.
/// - [`MissingCallbackUrl`](OAuthError::MissingCallbackUrl): The required callback URL for EVE Online OAuth2 was not provided.
/// - [`JwtKeyCacheError`](OAuthError::JwtKeyCacheError): An error occurred while retrieving JWT keys from the cache.
///
/// # Usage
/// These errors are typically returned when:
/// - Setting up the OAuth2 authentication process with `initiate_oauth_login`
/// - Attempting to exchange an authorization code for an access token
/// - Refreshing access tokens
#[derive(Error, Debug)]
pub enum OAuthError {
    /// Error returned when OAuth2 has not been configured for [`EsiClient`](crate::EsiClient).
    ///
    /// This error occurs when the [`EsiClient`](crate::EsiClient) is built without setting
    /// the client ID, client secret, and callback URL. This error occurs at runtime rather
    /// than setup because it does not yet know if you'll be using OAuth2 features unless
    /// you set at least one of OAuth2 related settings or try to use an OAuth2 related method.
    ///
    /// # Resolution
    /// To fix this configure your [`EsiClient`](crate::EsiClient) with the client ID, client secret,
    /// and callback URL from https://developers.eveonline.com/applications
    ///
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    #[error(
        "OAuth2 not configured for EsiClient\n\
        \n\
        To fix this configure your EsiClient with the client ID, client secret,\n\
        and callback URL from https://developers.eveonline.com/applications\n\
        \n\
        See this example: https://github.com/hyziri/eve_esi/blob/main/examples/sso.rs"
    )]
    OAuth2NotConfigured,

    /// Error returned when the ESI client ID is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client ID on the [`EsiClient`](crate::EsiClient).
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client_builder.client_id(client_id)`
    /// - You can obtain a client ID at: https://developers.eveonline.com/applications
    #[error(
        "Missing ESI client ID.\n\
        \n\
        To fix this:\n\
          - Set `esi_client_builder.client_id(client_id)`\n\
          - You can obtain a client ID at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientId,

    /// Error returned when the ESI client secret is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client secret on the [`EsiClient`](crate::EsiClient).
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client_builder.client_secret(client_secret)`
    /// - You can obtain a client secret at: https://developers.eveonline.com/applications
    #[error(
        "Missing ESI client secret.\n\
        \n\
        To fix this:\n\
          - Set `esi_client_builder.client_secret(client_secret)`\n\
          - You can obtain a client secret at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientSecret,

    /// Error returned when the ESI callback URL is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the callback URL on the [`EsiClient`](crate::EsiClient).
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client_builder.callback_url(callback_url)`
    /// - Ensure it matches the callback URL set at: https://developers.eveonline.com/applications
    #[error(
        "Missing ESI callback URL.\n\
        \n\
        To fix this:\n\
          - Set `esi_client_builder.callback_url(callback_url)`\n\
          - Ensure it matches the callback URL set at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingCallbackUrl,

    /// Error returned when the EVE OAuth2 callback URL is invalid.
    ///
    /// This error occurs when the callback url set using `esi_client_builder.callback_url(callback_url)`
    /// is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Validate the url set using `esi_client_builder.callback_url(callback_url)`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. `https://example.com/callback`
    #[error(
        "Invalid EVE OAuth2 callback URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the EsiClient\n\
          - Validate the url set using `esi_client_builder.callback_url(callback_url)`\n\
            is using a url that is correctly formatted\n\
            e.g. https://example.com/callback\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidCallbackUrl,

    /// Error updating JWT key cache to validate tokens
    ///
    /// Occurs when the JWT key cache for [`EsiClient`](crate::EsiClient) either is empty or
    /// contains expired keys and the attempt fetch new JWT keys fails.
    #[error("JWT key cache error: {0}")]
    JwtKeyCacheError(String),

    /// Error when JWT key refresh is still in cooldown
    ///
    /// If a recent set of attempts to refresh JWT key cache was made and all retries failed, a 60
    /// second cooldown period will be active until the next set of attempts.
    #[error("JWT key cache refresh cooldown still active: {0}")]
    JwtKeyRefreshCooldown(String),

    /// Errors types returned when an OAuth2 token request fails.
    ///
    /// For a more detailed explanation of the error, see the [`RequestTokenError`] enum.
    #[error("OAuth2 token error: {0:?}")]
    TokenError(
        RequestTokenError<
            HttpClientError<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
}

/// Error types related to the usage of [OAuth2ConfigBuilder](super::config::builder::OAuth2ConfigBuilder)
///
/// This enum represents the various errors which could occur due to an improperly configured
/// [OAuth2ConfigBuilder](super::config::builder::OAuth2ConfigBuilder)] It provides detailed error messages
/// with instructions on how to resolve each issue.
///
/// See the [module-level documentation](self) for an overview and usage example.
///
/// # Variants
/// - [`InvalidAuthUrl`](OAuthConfigBuilderError::InvalidAuthUrl): EVE OAuth2 authorization URL is in an invalid format.
/// - [`InvalidTokenUrl](OAuthConfigBuilderError::InvalidTokenUrl): EVE OAuth2 token URL is in an invalid format.
/// - [`InvalidJwkUrl](OAuthConfigBuilderError::InvalidJwkUrl): EVE OAuth2 JWK URL is in an invalid format.
/// - [`InvalidBackgroundRefreshThreshold`](OAuthConfigBuilderError::InvalidBackgroundRefreshThreshold): JWT key cache
///   background refresh threshold percentage is not between 0 and 100
///
/// # Usage
/// These errors are typically returned when improperly configuring an
/// [OAuth2ConfigBuilder](super::config::builder::OAuth2ConfigBuilder) which would cause calling
/// the `build` method to fail rather than successfully building an [OAuth2Config](super::OAuth2Config).
#[derive(Error, Debug)]
pub enum OAuthConfigError {
    /// EVE OAuth2 authentication URL is invalid.
    ///
    /// This error occurs when the auth url is changed from the default URL
    /// using [`super::OAuth2Config`] and is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the default config
    /// - Validate the url set using [`super::OAuth2Config`]
    ///   is using a url that is correctly formatted.
    ///
    ///   e.g. `https://login.eveonline.com/v2/oauth/authorize`
    #[error(
        "Invalid EVE OAuth2 URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the default config\n\
          - Validate the url set using [`super::OAuth2Config`]\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/authorize"
    )]
    InvalidAuthUrl,

    /// EVE OAuth2 token URL is invalid.
    ///
    /// This error occurs when the token url is changed from the default URL
    /// using [`super::OAuth2Config`] and is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the default config
    /// - Validate the url set using [`super::OAuth2Config`]
    ///   is using a url that is correctly formatted.
    ///
    ///   e.g. `https://login.eveonline.com/v2/oauth/token`
    #[error(
        "Invalid EVE OAuth2 token URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the default config\n\
          - Validate the url set using [`super::OAuth2Config`]\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/token"
    )]
    InvalidTokenUrl,

    /// JWT key cache background refresh threshold percentage is not between 0 and 100
    ///
    /// This error occurs when the background refresh threshold percentage used to
    /// determine when to proactively refresh the JWT key cache is configured to a value
    /// that is not between 0 and 100.
    ///
    /// Extreme values would either cause the refresh to trigger in quick succession if set
    /// too low or to never trigger if set too high.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default percentage provided by the default config
    /// - Validate the percentage set using [`super::OAuth2Config`]
    ///   is between 0 and 100.
    #[error(
        "Invalid JWT key cache background refresh threshold:\n\
        \n\
        To fix this:\n\
          - Use the default percentage provided by the default config\n\
          - Validate the percentage set using [`super::OAuth2Config`]
            is between 0 and 100."
    )]
    InvalidBackgroundRefreshThreshold,
}
