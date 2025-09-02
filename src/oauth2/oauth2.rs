//! Provides [`OAuth2Api`] for accessing OAuth2 related methods for the [`EsiClient`]
//!
//! The [`OAuth2Api`] struct provides access to oauth2 related methods for
//! the [`EsiClient`] using the [`EsiClient::oauth2`] method.
//!
//! For more details regarding using OAuth2 with the eve_esi crate, see [module-level documentation](super)

use crate::EsiClient;

/// Provides methods for accessing OAuth2-related endpoints of EVE Online's API.
///
/// The [`OAuth2Api`] struct acts as an interface for retrieving data from EVE Online's OAuth2 endpoints
/// It requires an [`EsiClient`] for making HTTP requests to the endpoints and managing JWT keys to validate tokens.
///
/// See the [module-level documentation](self) for an overview and usage example.
pub struct OAuth2Api<'a> {
    pub(super) client: &'a EsiClient,
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
