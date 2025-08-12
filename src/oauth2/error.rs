//! Errors that can occur during EVE Online's OAuth2 authentication process.
//!
//! This enum represents various error conditions that may arise when attempting to
//! authenticate with EVE Online's OAuth2 process. It provides detailed error messages
//! with instructions on how to resolve each issue.
//!
//! # Variants
//! - `MissingClientId` - The required client ID for EVE Online OAuth2 was not provided.
//! - `MissingClientSecret` - The required client secret for EVE Online OAuth2 was not provided.
//! - `MissingCallbackUrl` - The required callback URL for EVE Online OAuth2 was not provided.
//! - `InvalidAuthUrl` - The provided EVE OAuth2 authorization URL is in an invalid format.
//! - `InvalidTokenUrl` - The provided EVE OAuth2 token URL is in an invalid format.
//! - `InvalidCallbackUrl` - The provided EVE OAuth2 callback URL is in an invalid format.
//! - `CacheError` - An error occurred while retrieving JWT keys from the cache.
//!
//! # Usage
//! These errors are typically returned when:
//! - Setting up the OAuth2 authentication process with `initiate_oauth_login`
//! - Attempting to exchange an authorization code for an access token
//! - Refreshing OAuth2 tokens
//!
//! # Example
//! ```
//! use eve_esi::error::{EsiError, OAuthError};
//! use eve_esi::oauth2::ScopeBuilder;
//!
//! let esi_client = eve_esi::EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .build()
//!     .expect("Failed to build EsiClient");
//!
//! // Missing client ID will cause an error
//! let scopes = ScopeBuilder::new()
//!     .public_data()
//!     .build();
//! let result = esi_client.initiate_oauth_login(scopes);
//!
//! assert!(matches!(result, Err(EsiError::OAuthError(OAuthError::OAuth2NotConfigured))));
//! ```

use oauth2::basic::BasicErrorResponseType;
use oauth2::{HttpClientError, RequestTokenError, StandardErrorResponse};
use thiserror::Error;

/// Error types related to OAuth2 authentication for the EVE ESI client library.
///
/// This module defines the [`OAuthError`] type, providing structured and descriptive
/// error handling for OAuth2 authentication.
///
/// See the [module-level documentation](self) for an overview and usage example.
#[derive(Error, Debug)]
pub enum OAuthError {
    /// Error returned when OAuth2 has not been configured for `EsiClient`.
    ///
    /// This error occurs when the `EsiClient` is built without setting
    /// the client ID, client secret, and redirect URI.
    ///
    /// # Resolution
    /// To fix this configure your EsiClient with the client ID, client secret,
    /// and callback URL from https://developers.eveonline.com/applications
    ///
    /// ```
    /// let esi_client = eve_esi::EsiClient::builder()
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
        See this example: https://github.com/hyziri/eve_esi/blob/main/examples/sso.rs\n\
        "
    )]
    OAuth2NotConfigured,

    /// Error returned when the ESI client ID is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client ID on the EsiClient.
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client.set_client_id(client_id)`
    /// - You can obtain a client ID at:
    ///   https://developers.eveonline.com/applications
    #[error(
        "Missing ESI client ID.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_client_id(client_id)`\n\
          - You can obtain a client ID at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientId,

    /// Error returned when the ESI client secret is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client secret on the EsiClient.
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client.set_client_secret(client_secret)`
    /// - You can obtain a client secret at:
    ///   https://developers.eveonline.com/applications
    #[error(
        "Missing ESI client secret.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_client_secret(client_secret)`\n\
          - You can obtain a client secret at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientSecret,

    /// Error returned when the ESI callback URL is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the callback URL on the EsiClient.
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client.set_callback_url(callback_url)`
    /// - Ensure it matches the callback URL set at:
    ///   https://developers.eveonline.com/applications
    #[error(
        "Missing ESI callback URL.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_callback_url(callback_url)`\n\
          - Ensure it matches the callback URL set at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingCallbackUrl,

    /// Error returned when the EVE OAuth2 URL is invalid.
    ///
    /// This error occurs when the `esi_client.eve_auth_url` variable is changed
    /// from the default URL provided by the EsiClient and the url is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the EsiClient
    /// - Validate the url set using `esi_client.eve_auth_url = auth_url`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. https://login.eveonline.com/v2/oauth/authorize
    #[error(
        "Invalid EVE OAuth2 URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the EsiClient\n\
          - Validate the url set using `esi_client.eve_auth_url = auth_url`\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/authorize\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidAuthUrl,

    /// Error returned when the EVE OAuth2 token URL is invalid.
    ///
    /// This error occurs when the `esi_client.eve_token_url` variable is changed
    /// from the default URL provided by the EsiClient and the url is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the EsiClient
    /// - Validate the url set using `esi_client.eve_token_url = token_url`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. https://login.eveonline.com/v2/oauth/token
    #[error(
        "Invalid EVE OAuth2 token URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the EsiClient\n\
          - Validate the url set using `esi_client.eve_token_url = token_url`\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/token\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidTokenUrl,

    /// Error returned when the EVE OAuth2 callback URL is invalid.
    ///
    /// This error occurs when the callback url set using `esi_client.set_callback_url(callback_url)`
    /// is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Validate the url set using `esi_client.set_callback_url(callback_url)`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. https://example.com/callback
    #[error(
        "Invalid EVE OAuth2 callback URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the EsiClient\n\
          - Validate the url set using `esi_client.set_callback_url(callback_url)`\n\
            is using a url that is correctly formatted\n\
            e.g. https://example.com/callback\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidCallbackUrl,

    /// Error when JWT keys can't be retrieved from the cache despite being saved earlier.
    ///
    /// This should never happen as keys are always updated in the cache after fetching.
    #[error("An error occured while retrieving JWT keys from the cache.")]
    CacheError,

    /// Errors types returned when an OAuth2 token request fails.
    ///
    /// For a more detailed explanation of the error, see the `RequestTokenError` enum.
    #[error("OAuth2 token error: {0:?}")]
    TokenError(
        RequestTokenError<
            HttpClientError<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
}
