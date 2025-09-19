//! # EVE ESI OAuth2 Errors
//!
//! Provides an enum for runtime related OAuth2 errors, [`OAuthError`], which provides
//! detailed error messages as well as instructions on how to
//! resolve the issues which could occur.
//!
//! These errors are typically returned when handling JWT tokens for the SSO (single sign-on) login flow.
//! Possible errors could be not having the [`Client`](crate::Client) configured for OAuth2, an issue validating
//! a JWT token, or an issue fetching the JWT keys used to validate the token.
//!
//! For an overview & usage examples of OAuth2 with the `eve_esi` crate, see the [module-level documentation](super)
//!
//! ## Variants
//! ### Configuration Error
//! - [`OAuthError::OAuth2NotConfigured`]: Error returned when OAuth2 has not been configured for [`Client`](crate::Client).
//!
//! ### JWT Key Refresh Errors
//! - [`OAuthError::JwtKeyRefreshTimeout`]: Error when waiting for another thread to refresh JWT key cache times out
//! - [`OAuthError::JwtKeyRefreshFailure`]: Error when waiting for another thread to refresh JWT key cache fails
//! - [`OAuthError::JwtKeyRefreshCooldown`]: Error when JWT key refresh is still in cooldown
//!
//! ### JWT Token Errors
//! - [`OAuthError::RequestTokenError`]: Error when an OAuth2 token fetch request fails
//! - [`OAuthError::ValidateTokenError`]: Error when JWT key refresh is still in cooldown
//! - [`OAuthError::NoValidKeyFound`]: Error returned when JWT key cache does not have the ES256 token key needed for validation
//! - [`OAuthError::CharacterIdParseError]: Error when failing to parse character ID from JWT token claims
//!
//! ## Usage Example
//! ```
//! let esi_client = eve_esi::Client::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     // Don't set .client_id()
//!     // Don't set .client_secret()
//!     // Don't set .callback_url()
//!     .build()
//!     .expect("Failed to build Client");
//!
//! // Using OAuth2 without configuring required settings causes a runtime error.
//! let scopes = eve_esi::ScopeBuilder::new()
//!     .public_data()
//!     .build();
//! let result = esi_client.oauth2().login_url(scopes);
//!
//! assert!(matches!(result, Err(eve_esi::Error::OAuthError(eve_esi::OAuthError::OAuth2NotConfigured))));
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
#[derive(Error, Debug)]
pub enum OAuthError {
    /// Error returned when OAuth2 has not been configured for [`Client`](crate::Client).
    ///
    /// This error occurs when the [`Client`](crate::Client) is built without setting
    /// the client ID, client secret, and callback URL. This error occurs at runtime rather
    /// than setup because it does not yet know if you'll be using OAuth2 features unless
    /// you set at least one of OAuth2 related settings or try to use an OAuth2 related method.
    ///
    /// # Resolution
    /// To fix this configure your [`Client`](crate::Client) with the `client_id`, `client_secret`,
    /// and `callback_url` from <https://developers.eveonline.com/applications>.
    ///
    /// ```
    /// use eve_esi::Client;
    ///
    /// let esi_client = Client::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback")
    ///     .build()
    ///     .expect("Failed to build Client");
    /// ```
    #[error(
        "OAuth2 not configured for Client\n\
        \n\
        To fix this configure your Client with the client ID, client secret,\n\
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

    /// Error when an OAuth2 token fetch request fails
    ///
    /// For a more detailed explanation of the error, see the [`RequestTokenError`] enum.
    #[error("OAuth2 token error: {0:?}")]
    RequestTokenError(
        RequestTokenError<
            HttpClientError<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),

    /// Error type returned when OAuth2 token validation fails
    ///
    /// For a more detailed explanation of the error, see the [`jsonwebtoken::errors::Error`] enum.
    #[error("Validate token error: {0:?}")]
    ValidateTokenError(jsonwebtoken::errors::Error),

    /// Error returned when JWT key cache does not have the ES256 token key needed for validation
    ///
    /// This would be an issue with the jwt key cache not being empty nor expired but only having an ES256 key instead
    /// of both an ES256 and RS256 key as expected to be returned by EVE Online's JWT key API.
    #[error("No valid token key for validation found in cache: {0:?}")]
    NoValidKeyFound(String),

    /// Error when attempting to fetch from an authenticated route with an expired access token
    ///
    /// See [`crate::oauth2::token`] docs for instructions on how to refresh an expired token.
    #[error("Access token is expired\n
        \n
        See instructions on how to refresh an expired token here: <https://docs.rs/eve_esi/latest/eve_esi/oauth2/index.html>")]
    AccessTokenExpired(),

    /// Error when attempting to fetch from an authenticated route without the required scopes
    ///
    /// You will need to update your application at <https://developers.eveonline.com/applications>
    /// to include the missing scopes.
    #[error(
        "Missing required scopes for access token\n
        \n\
        Update your application at <https://developers.eveonline.com/applications>
        to include the missing scopes:\n
        {0:?}"
    )]
    AccessTokenMissingScopes(Vec<String>),

    /// Error when failing to parse character ID from JWT token claims
    ///
    /// This would be an internal error in this crate, should it occur please submit an
    /// issue on this crate's repository. This would only happen if EVE Online changes the
    /// format of the sub field in their JWT token claims.
    ///
    /// Returned when using [`crate::model::oauth2::EveJwtClaims::character_id`] method.
    #[error("Failed to parse character ID from EveJwtClaims due to error: {0:?}")]
    CharacterIdParseError(String),
}
