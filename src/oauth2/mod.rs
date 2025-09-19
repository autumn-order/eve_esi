//! # EVE ESI OAuth2
//!
//! This module provides methods for initiating and managing the EVE Online OAuth2 authentication process.
//! It includes functionality for generating login URLs to initiate the authentication process, building scopes for authorization, and managing tokens.
//!
//! Default settings for OAuth2 such as JWT key cache handling used to validate tokens or
//! the endpoints used for EVE OAuth2 can be overridden using the [`Config`](crate::Config).
//!
//! ## References
//! - <https://developers.eveonline.com/docs/services/sso/>
//!
//! ## Modules
//!
//! - [`login`]: Methods to begin the OAuth2 login process
//! - [`token`]: Methods to retrieve, validate, & refresh OAuth2 tokens
//! - [`scope`]: Builder to create scopes to request during the login process
//! - [`jwk`]: Methods to handle JSON web keys used to validate authentication tokens
//! - [`error`]: Error enum for any OAuth2 related errors.

pub mod error;
pub mod jwk;
pub mod login;
pub mod oauth2;
pub mod scope;
pub mod token;

pub use oauth2::OAuth2Api;
pub use scope::ScopeBuilder;

pub(crate) mod client;
