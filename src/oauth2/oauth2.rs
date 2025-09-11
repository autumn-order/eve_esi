//! Provides [`OAuth2Api`] for accessing OAuth2 related methods for the [`Client`]
//!
//! The [`OAuth2Api`] struct provides access to oauth2 related methods for
//! the [`Client`] using the [`Client::oauth2`] method.
//!
//! For more details regarding using OAuth2 with the eve_esi crate, see [module-level documentation](super)

use crate::Client;

/// Provides methods for accessing OAuth2-related endpoints of EVE Online's API.
///
/// The [`OAuth2Api`] struct acts as an interface for retrieving data from EVE Online's OAuth2 endpoints
/// It requires an [`Client`] for making HTTP requests to the endpoints and managing JWT keys to validate tokens.
///
/// See the [module-level documentation](self) for an overview and usage example.
pub struct OAuth2Api<'a> {
    pub(super) client: &'a Client,
}

impl Client {
    /// Access to EVE Online's OAuth2 endpoints
    ///
    /// Returns an API client for interacting with the OAuth2 endpoints.
    pub fn oauth2(&self) -> self::OAuth2Api<'_> {
        self::OAuth2Api::new(self)
    }
}

impl<'a> OAuth2Api<'a> {
    /// Creates a new instance of [`OAuth2Api`]
    ///
    /// # Arguments
    /// - `client` (&'a [`Client`]) used for making HTTP requests to EVE Online's ESI & OAuth2
    ///   endpoints and providing the JWT key caching & refresh handling used to validate tokens.
    ///
    /// # Returns
    /// - `Self`: A new instance of [`OAuth2Api`].
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
