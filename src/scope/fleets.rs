//! # EVE ESI Fleets Scopes
//!
//! This module provides a type-safe way to add fleet-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                        |
//! | ----------------------------------------- | ------------------------------------------------------------------ |
//! | [`FleetsScopes::new`]                     | Creates a new instance of [`FleetsScopes`]                         |
//! | [`FleetsScopes::all`]                     | Creates a new instance of [`FleetsScopes`] with all scopes applied |

/// Struct with methods for listing fleet scopes to request for OAuth2
pub struct FleetsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for FleetsScopes {
    /// Create a default instance of [`FleetsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl FleetsScopes {
    /// Create a new instance of [`FleetsScopes`]
    pub fn new() -> Self {
        FleetsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`FleetsScopes`] with all scopes applied
    pub fn all() -> Self {
        FleetsScopes::new()
    }
}
