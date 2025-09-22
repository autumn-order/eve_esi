//! # EVE ESI Fittings Scopes
//!
//! This module provides a type-safe way to add fitting-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                      | Description                                                          |
//! | ------------------------------------------- | -------------------------------------------------------------------- |
//! | [`FittingsScopes::new`]                     | Creates a new instance of [`FittingsScopes`]                         |
//! | [`FittingsScopes::all`]                     | Creates a new instance of [`FittingsScopes`] with all scopes applied |

/// Struct with methods for listing fitting scopes to request for OAuth2
pub struct FittingsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for FittingsScopes {
    /// Create a default instance of [`FittingsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl FittingsScopes {
    /// Create a new instance of [`FittingsScopes`]
    pub fn new() -> Self {
        FittingsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`FittingsScopes`] with all scopes applied
    pub fn all() -> Self {
        FittingsScopes::new()
    }
}

#[cfg(test)]
mod fittings_scopes_tests {
    use crate::scope::FittingsScopes;

    /// Tests initializing a default instance of [`FittingsScopes`]
    #[test]
    fn test_fittings_scopes_default() {
        let fittings_scopes = FittingsScopes::default();

        assert_eq!(fittings_scopes.scopes.len(), 0)
    }
}
