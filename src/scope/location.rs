//! # EVE ESI Location Scopes
//!
//! This module provides a type-safe way to add location-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                          |
//! | ----------------------------------------- | -------------------------------------------------------------------- |
//! | [`LocationScopes::new`]                   | Creates a new instance of [`LocationScopes`]                         |
//! | [`LocationScopes::all`]                   | Creates a new instance of [`LocationScopes`] with all scopes applied |
//! | [`LocationScopes::read_location`]         | Read access to character's current location                          |
//! | [`LocationScopes::read_online`]           | Read access to characer's online status                              |
//! | [`LocationScopes::read_ship_type`]        | Read access to character's ship type                                 |

/// Read access to character's current location
pub const READ_LOCATION: &str = "esi-location.read_location.v1";
/// Read access to characer's online status
pub const READ_ONLINE: &str = "esi-location.read_online.v1";
/// Read access to character's ship type
pub const READ_SHIP_TYPE: &str = "esi-location.read_ship_type.v1";

/// Struct with methods for listing location scopes to request for OAuth2
pub struct LocationScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for LocationScopes {
    /// Create a default instance of [`LocationScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl LocationScopes {
    /// Create a new instance of [`LocationScopes`]
    pub fn new() -> Self {
        LocationScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`LocationScopes`] with all scopes applied
    pub fn all() -> Self {
        LocationScopes::new()
            .read_location()
            .read_online()
            .read_ship_type()
    }

    /// Read access to character's current location
    ///
    /// Adds the `esi-location.read_location.v1` scope
    pub fn read_location(mut self) -> Self {
        self.scopes.push(READ_LOCATION.to_string());
        self
    }

    /// Read access to characer's online status
    ///
    /// Adds the `esi-location.read_online.v1` scope
    pub fn read_online(mut self) -> Self {
        self.scopes.push(READ_ONLINE.to_string());
        self
    }

    /// Read access to character's ship type
    ///
    /// Adds the `esi-location.read_ship_type.v1` scope
    pub fn read_ship_type(mut self) -> Self {
        self.scopes.push(READ_SHIP_TYPE.to_string());
        self
    }
}

#[cfg(test)]
mod location_scopes_tests {
    use crate::scope::LocationScopes;

    /// Tests initializing a default instance of [`LocationScopes`]
    #[test]
    fn test_location_scopes_default() {
        let location_scopes = LocationScopes::default();

        assert_eq!(location_scopes.scopes.len(), 0)
    }
}
