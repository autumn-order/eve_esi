//! # EVE ESI Alliance Scopes
//!
//! This module provides a type-safe way to add alliance-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                       | Description                                                           |
//! | -------------------------------------------- | --------------------------------------------------------------------- |
//! | [`AlliancesScopes::new`]                     | Creates a new instance of [`AlliancesScopes`]                         |
//! | [`AlliancesScopes::all`]                     | Creates a new instance of [`AlliancesScopes`] with all scopes applied |
//! | [`AlliancesScopes::read_contacts`]           | Read access to alliance contact information                           |

/// Read access to alliance contact information
pub const READ_CONTACTS: &str = "esi-alliances.read_contacts.v1";

/// Struct with methods for listing alliance scopes to request for OAuth2
pub struct AlliancesScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for AlliancesScopes {
    /// Create a default instance of [`AlliancesScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl AlliancesScopes {
    /// Create a new instance of [`AlliancesScopes`]
    pub fn new() -> Self {
        AlliancesScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`AlliancesScopes`] with all scopes applied
    pub fn all() -> Self {
        AlliancesScopes::new().read_contacts()
    }

    /// Read access to alliance contact information
    ///
    /// Adds the `esi-alliances.read_contacts.v1` scope
    pub fn read_contacts(mut self) -> Self {
        self.scopes.push(READ_CONTACTS.to_string());
        self
    }
}

#[cfg(test)]
mod alliances_scopes_tests {
    use crate::scope::AlliancesScopes;

    /// Tests initializing a default instance of [`AlliancesScopes`]
    #[test]
    fn test_alliances_scopes_default() {
        let alliances_scopes = AlliancesScopes::default();

        assert_eq!(alliances_scopes.scopes.len(), 0)
    }
}
