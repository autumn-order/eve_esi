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
//! | [`AssetsScopes::read_assets`]             | Read access to character's assets                                  |
//! | [`AssetsScopes::read_corporation_assets`] | Read access to corporation's assets                                |

/// Read access to character's assets
pub const READ_ASSETS: &str = "esi-assets.read_assets.v1";
/// Read access to corporation's assets
pub const READ_CORPORATION_ASSETS: &str = "esi-assets.read_corporation_assets.v1";

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
        AssetsScopes::new().read_assets().read_corporation_assets()
    }

    /// Read access to character's assets
    ///
    /// Adds the `esi-assets.read_assets.v1` scope
    pub fn read_assets(mut self) -> Self {
        self.scopes.push(READ_ASSETS.to_string());
        self
    }

    /// Read access to corporation's assets
    ///
    /// Adds the `esi-assets.read_corporation_assets.v1` scope
    pub fn read_corporation_assets(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_ASSETS.to_string());
        self
    }
}

#[cfg(test)]
mod assets_scopes_tests {
    use crate::scope::AssetsScopes;

    /// Tests initializing a default instance of [`AssetsScopes`]
    #[test]
    fn test_assets_scopes_default() {
        let assets_scopes = AssetsScopes::default();

        assert_eq!(assets_scopes.scopes.len(), 0)
    }
}
