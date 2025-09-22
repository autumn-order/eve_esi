//! # EVE ESI Clones Scopes
//!
//! This module provides a type-safe way to add clone-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                        |
//! | ----------------------------------------- | ------------------------------------------------------------------ |
//! | [`ClonesScopes::new`]                     | Creates a new instance of [`ClonesScopes`]                         |
//! | [`ClonesScopes::all`]                     | Creates a new instance of [`ClonesScopes`] with all scopes applied |

/// Struct with methods for listing clone scopes to request for OAuth2
pub struct ClonesScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for ClonesScopes {
    /// Create a default instance of [`ClonesScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl ClonesScopes {
    /// Create a new instance of [`ClonesScopes`]
    pub fn new() -> Self {
        ClonesScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`ClonesScopes`] with all scopes applied
    pub fn all() -> Self {
        ClonesScopes::new()
    }
}
