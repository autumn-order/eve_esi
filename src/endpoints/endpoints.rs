//! Implements ESI endpoints for usage with the [`EsiClient`]
//!
//! For details on usage see [module-level documentation](super)

use crate::EsiClient;

impl EsiClient {
    /// Access to Alliance ESI endpoints
    ///
    /// Returns an API client for interacting with alliance-related endpoints.
    pub fn alliance(&self) -> super::alliance::AllianceApi<'_> {
        super::alliance::AllianceApi::new(self)
    }

    /// Access to Character ESI endpoints
    ///
    /// Returns an API client for interacting with character-related endpoints.
    pub fn character(&self) -> super::character::CharacterApi<'_> {
        super::character::CharacterApi::new(self)
    }

    /// Access to Corporation ESI endpoints
    ///
    /// Returns an API client for interacting with corporation-related endpoints.
    pub fn corporation(&self) -> super::corporation::CorporationApi<'_> {
        super::corporation::CorporationApi::new(self)
    }
}
