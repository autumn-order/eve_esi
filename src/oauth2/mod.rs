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
//! - [`jwk`]: Methods to handle JSON web keys used to validate authentication tokens
//! - [`error`]: Error enum for any OAuth2 related errors.
//!
//! ## Usage Examples
//!
//! - [Creating a login URL for single sign-on (OAuth2)](crate::oauth2::login)
//! - [Fetching an access token](crate::oauth2::token)
//! - [Validating an access token](crate::oauth2::token)
//! - [Refreshing an access token](crate::oauth2::token)

pub mod error;
pub mod jwk;
pub mod login;
pub mod token;

pub(crate) mod client;

use crate::Client;

/// Provides methods for accessing OAuth2-related endpoints of EVE Online's API.
///
/// The [`OAuth2Api`] struct acts as an interface for retrieving data from EVE Online's OAuth2 endpoints
/// It requires an [`Client`] for making HTTP requests to the endpoints and managing JWT keys to validate tokens.
///
/// See the [module-level documentation](self) for an overview and usage example.
pub struct OAuth2Endpoints<'a> {
    pub(super) client: &'a Client,
}

impl Client {
    /// Access to EVE Online's OAuth2 endpoints
    ///
    /// Returns an API client for interacting with the OAuth2 endpoints.
    pub fn oauth2(&self) -> self::OAuth2Endpoints<'_> {
        self::OAuth2Endpoints::new(self)
    }
}

impl<'a> OAuth2Endpoints<'a> {
    /// Creates a new instance of [`OAuth2Api`]
    ///
    /// # Arguments
    /// - `client` (&'a [`Client`]) used for making HTTP requests to EVE Online's ESI & OAuth2
    ///   endpoints and providing the JWT key caching & refresh handling used to validate tokens.
    ///
    /// # Returns
    /// - `Self`: A new instance of [`OAuth2Api`].
    pub(self) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
