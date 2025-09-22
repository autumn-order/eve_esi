//! # EVE ESI Assets Scopes
//!
//! This module provides a type-safe way to add asset-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                        |
//! | ----------------------------------------- | ------------------------------------------------------------------ |
//! | [`AssetsScopes::new`]                     | Creates a new instance of [`AssetsScopes`]                         |
//! | [`AssetsScopes::all`]                     | Creates a new instance of [`AssetsScopes`] with all scopes applied |

/// Struct with methods for listing asset scopes to request for OAuth2
pub struct AssetsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for AssetsScopes {
    /// Create a default instance of [`AssetsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl AssetsScopes {
    /// Create a new instance of [`AssetsScopes`]
    pub fn new() -> Self {
        AssetsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`AssetsScopes`] with all scopes applied
    pub fn all() -> Self {
        AssetsScopes::new()
    }
}
