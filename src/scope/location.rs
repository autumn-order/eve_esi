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
    }
}
