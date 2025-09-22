//! # EVE ESI Assets Scopes
//!
//! This module provides a type-safe way to add asset-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                   | Description                                                       |
//! | ---------------------------------------- | ----------------------------------------------------------------- |
//! | [`AssetScopes::new`]                     | Creates a new instance of [`AssetScopes`]                         |
//! | [`AssetScopes::all`]                     | Creates a new instance of [`AssetScopes`] with all scopes applied |

/// Struct with methods for listing asset scopes to request for OAuth2
pub struct AssetScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for AssetScopes {
    /// Create a default instance of [`AssetScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl AssetScopes {
    /// Create a new instance of [`AssetScopes`]
    pub fn new() -> Self {
        AssetScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`AssetScopes`] with all scopes applied
    pub fn all() -> Self {
        AssetScopes::new()
    }
}
