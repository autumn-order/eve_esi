//! # EVE ESI API
//!
//! Provides the [`EsiApi`] struct for the [`Client`] which implmements the underlying utility methods
//! for making public & authenticated ESI endpoint requests.
//!
//! See the [module-level documentation](super) for an overview, methods, & usage example.

use crate::Client;

/// Provides utility methods for making requests EVE Online's ESI endpoints
///
/// See the [module-level documentation](super) for an overview, methods, & usage example.
pub struct EsiApi<'a> {
    pub(crate) client: &'a Client,
}

impl Client {
    /// Access to utility functions to make ESI requests
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
    pub fn esi(&self) -> self::EsiApi<'_> {
        self::EsiApi::new(self)
    }
}

impl<'a> EsiApi<'a> {
    /// Creates a new instance of [`EsiApi`]
    fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
