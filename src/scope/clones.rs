//! # EVE ESI Clones Scopes
//!
//! This module provides a type-safe way to add clone-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                   | Description                                                       |
//! | ---------------------------------------- | ----------------------------------------------------------------- |
//! | [`CloneScopes::new`]                     | Creates a new instance of [`CloneScopes`]                         |
//! | [`CloneScopes::all`]                     | Creates a new instance of [`CloneScopes`] with all scopes applied |

/// Struct with methods for listing clone scopes to request for OAuth2
pub struct CloneScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for CloneScopes {
    /// Create a default instance of [`CloneScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl CloneScopes {
    /// Create a new instance of [`CloneScopes`]
    pub fn new() -> Self {
        CloneScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`CloneScopes`] with all scopes applied
    pub fn all() -> Self {
        CloneScopes::new()
    }
}
