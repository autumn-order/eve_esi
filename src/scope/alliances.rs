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
