//! Error types for the EVE ESI client library.
//!
//! This module defines the top-level error types used throughout the crate, providing
//! structured and descriptive error handling for both OAuth2 authentication and HTTP requests.
//!
//! # Overview
//!
//! The primary error type is [`EsiError`], which encapsulates all possible error conditions
//! that may arise when interacting with the EVE ESI API. This includes errors related to
//! configuration (see [`EsiConfigError`]), OAuth2 authentication (see [`OAuthError`]), and
//! HTTP request failures (see [`reqwest::Error`]).
//!
//! By using these error types, consumers of the library can match on specific error variants
//! to implement granular error handling or simply handle errors at a higher level.
//!
//! See the documentation for [`EsiError`] and [`EsiConfigError`] for more details on each error variant.
//!
//! # Example
//!
//! ```rust
//! use eve_esi::error::EsiError;
//! use eve_esi::oauth2::ScopeBuilder;
//! use eve_esi::EsiClient;
//!
//! // Don't set any OAuth2 related settings
//! let esi_client = EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .build()
//!     .expect("Failed to build EsiClient");
//!
//! let scopes = ScopeBuilder::new()
//!     .public_data()
//!     .build();
//!
//! // OAuth2 runtime error will be returned due to OAuth2 not being setup on client
//! let result = esi_client.oauth2().login_url(scopes);
//!
//! // Handle error types
//! match result {
//!     Ok(_) => { /* ... */ }
//!     Err(EsiError::OAuthError(oauth_err)) => {
//!         // Handle OAuth2-specific error
//!         println!("OAuth2 error: {oauth_err}");
//!     }
//!     // Additional EsiError types
//!     err => panic!("Unexpected error type: {:#?}", err)
//! }
//! ```

use thiserror::Error;

pub use crate::oauth2::error::OAuthError;

/// Runtime errors that can occur when using the EVE ESI client.
///
/// This is the top-level error type returned by most methods in this crate. It encapsulates
/// all possible error conditions, including OAuth2 authentication errors and HTTP request failures.
///
/// See the [module-level documentation](self) for an overview and usage example.
///
/// # Variants
/// - [`EsiConfigError`](EsiError::EsiConfigError) - Errors due to configuration issues when building an [`EsiClient`](crate::EsiClient)
///   or [`EsiConfig`](crate::config::EsiConfig)
/// - [`OAuthError`](EsiError::OAuthError) - Errors related to OAuth2 authentication. See [`OAuthError`] for details.
/// - [`ReqwestError`](EsiError::ReqwestError) - Errors that occur during HTTP requests. See [`reqwest::Error`] for details.
///
/// # Usage
/// You can match on [`EsiError`] to handle errors at a high level, or downcast to more specific
/// error types for granular handling.
///
#[derive(Error, Debug)]
pub enum EsiError {
    /// Config errors related to building a [`EsiConfig`](crate::EsiConfig) or [`EsiClient`](crate::EsiClient)
    ///
    /// For a more detailed description, see [`EsiConfigError`]
    #[error(transparent)]
    EsiConfigError(EsiConfigError),
    /// Runtime errors related to the EVE Online OAuth2 authentication process.
    ///
    /// For a more detailed description, see [`OAuthError`].
    #[error(transparent)]
    OAuthError(OAuthError),
    /// Errors that occur during HTTP requests.
    ///
    /// For a more detailed description, see [`reqwest::Error`].
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

/// Errors when building a new [`EsiClient`](crate::EsiClient) or [`EsiConfig`](crate::config::EsiConfig)
///
/// This enum represents the various errors which could occur due to an improper configuration such as an
/// improper URL format or an invalid JWT key background refresh threshold.
///
/// See the [module-level documentation](self) for an overview and usage example.
///
/// # Variants
/// - [`InvalidAuthUrl`](EsiConfigError::InvalidAuthUrl): EVE OAuth2 authorization URL is in an invalid URL format.
/// - [`InvalidTokenUrl](EsiConfigError::InvalidTokenUrl): EVE OAuth2 token URL is in an invalid URL format.
/// - [`InvalidBackgroundRefreshThreshold`](EsiConfigError::InvalidBackgroundRefreshThreshold): JWT key cache
///   background refresh threshold percentage is not between 0 and 100
///
/// # Usage
/// You can match on [`EsiConfigError`] to handle errors at a high level, or downcast to more specific
/// error types for granular handling.
#[derive(Error, Debug)]
pub enum EsiConfigError {
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

    /// EVE OAuth2 authentication URL is invalid.
    ///
    /// This error occurs when the auth url is changed from the default URL
    /// on [`EsiConfig`](crate::config::EsiConfig) and is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the default config
    /// - Validate the url set on [`EsiConfig`](crate::config::EsiConfig)
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
    /// on [`EsiConfig`](crate::config::EsiConfig) and is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the default config
    /// - Validate the url set on [`EsiConfig`](crate::config::EsiConfig)
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
    /// - Validate the percentage set on [`EsiConfig`](crate::config::EsiConfig)
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
