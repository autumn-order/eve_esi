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

/// Access to read alliance contact information
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
        AlliancesScopes::new()
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
