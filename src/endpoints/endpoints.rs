//! # EVE ESI Endpoint Implementations
//!
//! Implements ESI endpoint categories for the ESI [`Client`]
//!
//! For an overview & usage example, see the [endpoints module documentation](super)

use crate::Client;

impl Client {
    /// Access to Alliance ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with alliance-related endpoints.
    pub fn alliance(&self) -> super::alliance::AllianceApi<'_> {
        super::alliance::AllianceApi::new(self)
    }

    /// Access to Character ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with character-related endpoints.
    pub fn character(&self) -> super::character::CharacterApi<'_> {
        super::character::CharacterApi::new(self)
    }

    /// Access to Corporation ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with corporation-related endpoints.
    pub fn corporation(&self) -> super::corporation::CorporationApi<'_> {
        super::corporation::CorporationApi::new(self)
    }
}
