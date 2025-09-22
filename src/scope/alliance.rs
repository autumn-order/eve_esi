//! # EVE ESI Alliance Scopes
//!
//! This module provides a type-safe way to add alliance-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                      | Description                                                          |
//! | ------------------------------------------- | -------------------------------------------------------------------- |
//! | [`AllianceScopes::new`]                     | Creates a new instance of [`AllianceScopes`]                         |
//! | [`AllianceScopes::all`]                     | Creates a new instance of [`AllianceScopes`] with all scopes applied |

/// Struct with methods for listing alliance scopes to request for OAuth2
pub struct AllianceScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for AllianceScopes {
    /// Create a default instance of [`AllianceScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl AllianceScopes {
    /// Create a new instance of [`AllianceScopes`]
    pub fn new() -> Self {
        AllianceScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`AllianceScopes`] with all scopes applied
    pub fn all() -> Self {
        AllianceScopes::new()
    }
}
