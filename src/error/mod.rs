//! # EVE ESI Runtime & Config Errors
//!
//! This module defines the top-level error types used throughout the crate, providing
//! structured and descriptive error handling for both OAuth2 authentication and HTTP requests.
//!
//! # Overview
//!
//! The primary error type is [`enum@Error`], which encapsulates all possible error conditions
//! that may arise when interacting with the EVE ESI API. This includes errors related to
//! configuration (see [`ConfigError`]), OAuth2 authentication (see [`OAuthError`]), and
//! HTTP request failures (see [`reqwest::Error`]).
//!
//! By using these error types, consumers of the library can match on specific error variants
//! to implement granular error handling or simply handle errors at a higher level.
//!
//! See the documentation for [`enum@Error`] and [`ConfigError`] for more details on each error variant.
//!
//! ## Usage Example
//!
//! ```rust
//! // Don't set any OAuth2 related settings
//! let esi_client = eve_esi::Client::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .build()
//!     .expect("Failed to build Client");
//!
//! let scopes = eve_esi::ScopeBuilder::new()
//!     .public_data()
//!     .build();
//!
//! // OAuth2 runtime error will be returned due to OAuth2 not being setup on client
//! let result = esi_client.oauth2().login_url(scopes);
//!
//! // Handle error types
//! match result {
//!     Ok(_) => { /* ... */ }
//!     Err(eve_esi::Error::OAuthError(oauth_err)) => {
//!         // Handle OAuth2-specific error
//!         println!("OAuth2 error: {oauth_err}");
//!     }
//!     // Additional EsiError types
//!     err => panic!("Unexpected error type: {:#?}", err)
//! }
//! ```

use thiserror::Error;

mod config;
mod response;

pub use crate::oauth2::error::OAuthError;
pub use config::ConfigError;
pub use response::{EsiResponseError, EsiResponseErrorData};

/// Runtime errors that can occur when using the EVE ESI client.
///
/// This is the top-level error type returned by most methods in this crate. It encapsulates
/// all possible error conditions, including OAuth2 authentication errors and HTTP request failures.
///
/// See the [module-level documentation](self) for an overview and usage example.
#[derive(Error, Debug)]
pub enum Error {
    /// Config errors related to building a [`Config`](crate::Config) or [`Client`](crate::Client)
    ///
    /// For a more detailed description, see [`ConfigError`]
    #[error(transparent)]
    ConfigError(ConfigError),
    /// Runtime errors related to the EVE Online OAuth2 authentication process.
    ///
    /// For a more detailed description, see [`OAuthError`].
    #[error(transparent)]
    OAuthError(OAuthError),
    /// ESI API returned an error response (4xx or 5xx status code).
    ///
    /// Contains the error message from ESI along with cache and rate limit headers.
    #[error("ESI API error: {0}")]
    EsiResponseError(#[from] EsiResponseError),
    /// Errors that occur during HTTP requests.
    ///
    /// For a more detailed description, see [`reqwest::Error`].
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Errors related to parsing a URL for HTTP requests
    ///
    /// This would occur if a URL for making an ESI request is
    /// improperly formatted making it an invalid URL.
    ///
    /// For a more detailed description, see [`url::ParseError`].
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Errors related to serializing or deserializing JSON data
    ///
    /// This would occur if there is an issue serializing a Rust type
    /// to JSON for usage in URL query params for ESI endpoints.
    ///
    /// For a more detailed description, see [`serde_json::Error`].
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
