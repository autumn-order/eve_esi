//! # EVE ESI Fittings Scopes
//!
//! This module provides a type-safe way to add fitting-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                     | Description                                                         |
//! | ------------------------------------------ | ------------------------------------------------------------------- |
//! | [`FittingScopes::new`]                     | Creates a new instance of [`FittingScopes`]                         |
//! | [`FittingScopes::all`]                     | Creates a new instance of [`FittingScopes`] with all scopes applied |

/// Struct with methods for listing fitting scopes to request for OAuth2
pub struct FittingScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for FittingScopes {
    /// Create a default instance of [`FittingScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl FittingScopes {
    /// Create a new instance of [`FittingScopes`]
    pub fn new() -> Self {
        FittingScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`FittingScopes`] with all scopes applied
    pub fn all() -> Self {
        FittingScopes::new()
    }
}
