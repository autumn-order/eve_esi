//! Errors related to the OAuth2 portion of the EVE ESI crate
//!
//! Provides an enum for runtime related errors, [`OAuthError`], which provides
//! detailed error messages as well as instructions on how to
//! resolve the issues which could occur.
//!
//! # Error Types
//! - [`OAuthError`]: Runtime errors related to EVE OAuth2
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
//! let result = esi_client.oauth2().login_url(scopes);
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
/// - [`OAuth2NotConfigured`](OAuthError::OAuth2NotConfigured): Error returned when OAuth2 has not been configured for [`EsiClient`](crate::EsiClient).
/// - [`JwtKeyRefreshTimeout`](OAuthError::JwtKeyRefreshTimeout): Error when waiting for another thread to refresh JWT key cache times out
/// - [`JwtKeyRefreshFailure`](OAuthError::JwtKeyRefreshFailure): Error when waiting for another thread to refresh JWT key cache fails
/// - [`JwtKeyRefreshCooldown`](OAuthError::JwtKeyRefreshCooldown): Error when JWT key refresh is still in cooldown
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
    /// To fix this configure your [`EsiClient`](crate::EsiClient) with the `client_id`, `client_secret`,
    /// and `callback_url` from https://developers.eveonline.com/applications.
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

    /// Error when waiting for another thread to refresh JWT key cache times out
    ///
    /// Occurs when waiting for another thread to refresh the JWT key cache but wait time is over
    /// 5 seconds, causing a timeout error.
    #[error("JWT key refresh timeout: {0}")]
    JwtKeyRefreshTimeout(String),

    /// Error when waiting for another thread to refresh JWT key cache fails
    ///
    /// Occurs when waiting for another thread to refresh the JWT key cache but after receiving
    /// a notification that a refresh was completed but the cache is still empty or expired
    /// which means the fetch attempt likely failed.
    #[error("JWT key refresh failure: {0}")]
    JwtKeyRefreshFailure(String),

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
