//! Error types for the EVE ESI client library.
//!
//! This module defines the top-level error types used throughout the crate, providing
//! structured and descriptive error handling for both OAuth2 authentication and HTTP requests.
//!
//! # Overview
//!
//! The primary error type is [`EsiError`], which encapsulates all possible error conditions
//! that may arise when interacting with the EVE ESI API. This includes errors related to
//! OAuth2 authentication (see [`OAuthError`]) as well as HTTP request failures (see [`reqwest::Error`]).
//!
//! By using these error types, consumers of the library can match on specific error variants
//! to implement granular error handling or simply handle errors at a higher level.
//!
//! # Example
//!
//! ```rust
//! use eve_esi::error::EsiError;
//! use eve_esi::oauth2::ScopeBuilder;
//! use eve_esi::EsiClient;
//!
//! let esi_client = EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .build()
//!     .expect("Failed to build EsiClient");
//!
//! let scopes = ScopeBuilder::new()
//!     .public_data()
//!     .build();
//! let result = esi_client.oauth2().initiate_oauth_login(scopes);
//!
//! // Handle error types
//! match result {
//!     Ok(_) => { /* ... */ }
//!     Err(EsiError::OAuthError(auth_err)) => {
//!         // Handle OAuth-specific errors
//!         println!("OAuth error: {auth_err}");
//!     }
//!     Err(EsiError::ReqwestError(http_err)) => {
//!         // Handle HTTP errors
//!         println!("HTTP error: {http_err}");
//!     }
//!     // Additional EsiError types
//!     _ => todo!()
//! }
//! ```
//!
//! See the documentation for [`EsiError`] and [`OAuthError`] for more details on each error variant.

use thiserror::Error;

use crate::oauth2::error::OAuthConfigError;
pub use crate::oauth2::error::OAuthError;

/// Errors that can occur when using the EVE ESI client.
///
/// This is the top-level error type returned by most methods in this crate. It encapsulates
/// all possible error conditions, including OAuth2 authentication errors and HTTP request failures.
///
/// # Variants
/// - `OAuthError` - Errors related to OAuth2 authentication. See [`OAuthError`] for details.
/// - `ReqwestError` - Errors that occur during HTTP requests. See [`reqwest::Error`] for details.
///
/// # Usage
/// You can match on `EsiError` to handle errors at a high level, or downcast to more specific
/// error types for granular handling.
///
/// See the [module-level documentation](self) for an overview and usage example.
#[derive(Error, Debug)]
pub enum EsiError {
    /// Runtime errors related to the EVE Online OAuth2 authentication process.
    ///
    /// For a more detailed description, see [`OAuthError`].
    #[error(transparent)]
    OAuthError(OAuthError),
    /// Config errors related to building a [OAuth2Config](crate::oauth2::OAuth2Config)
    ///
    /// For a more detailed description, see [`OAuthConfigError`]
    #[error(transparent)]
    OAuthConfigError(OAuthConfigError),
    /// Errors that occur during HTTP requests.
    ///
    /// For a more detailed description, see [`ReqwestError`].
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
