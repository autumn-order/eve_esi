//! Methods for OAuth2 authentication with EVE Online SSO
//!
//! This module provides methods for initiating and managing the EVE Online OAuth2 authentication process
//! It includes functionality for generating login URLs to initiate the authentication process, building scopes for authorization, and managing tokens.
//!
//! # References
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! # Modules
//!
//! - [`auth`](crate::endpoints::auth)
//! - [`scope`](crate::endpoints::scope)
//! - [`token`](crate::endpoints::token)
//!
//! # Example
//! ```
//! let esi_client = eve_esi::EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .client_id("client_id")
//!     .client_secret("client_secret")
//!     .callback_url("http://localhost:8080/callback")
//!     .build()
//!     .expect("Failed to build EsiClient");
//!
//! let scopes = eve_esi::oauth2::ScopeBuilder::new()
//!     .public_data()
//!     .build();
//! let auth_data = esi_client
//!     .oauth2()
//!     .initiate_oauth_login(scopes)
//!     .expect("Failed to initiate OAuth login");
//!
//! println!("Login URL: {}", auth_data.login_url);
//! ```

pub mod error;
pub mod jwk;
pub mod login;
pub mod scope;
pub mod token;

pub(crate) mod client;

pub use scope::ScopeBuilder;

use crate::EsiClient;

/// Provides methods for accessing OAuth2-related endpoints of EVE Online's API.
///
/// The [`OAuth2Api`] struct acts as an interface for retrieving data from EVE Online's OAuth2 endpoints
/// It requires an [`EsiClient`] for making HTTP requests to the endpoints and managing JWT keys to validate tokens.
///
/// See the [module-level documentation](self) for an overview and usage example.
pub struct OAuth2Api<'a> {
    client: &'a EsiClient,
}

impl<'a> OAuth2Api<'a> {
    /// Creates a new instance of [`OAuth2Api`]
    ///
    /// # Arguments
    /// - `client: The [`EsiClient`] used for making HTTP requests to the ESI endpoints.
    ///
    /// # Returns
    /// - `Self`: A new instance of [`OAuth2Api`].
    pub fn new(client: &'a EsiClient) -> Self {
        Self { client }
    }
}

impl EsiClient {
    /// Access to EVE Online's OAuth2 endpoints
    ///
    /// Returns an API client for interacting with the OAuth2 endpoints.
    pub fn oauth2(&self) -> self::OAuth2Api<'_> {
        self::OAuth2Api::new(self)
    }
}
