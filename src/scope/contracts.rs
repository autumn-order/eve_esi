//! # EVE ESI Contracts Scopes
//!
//! This module provides a type-safe way to add contract-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                      | Description                                                          |
//! | ------------------------------------------- | -------------------------------------------------------------------- |
//! | [`ContractScopes::new`]                     | Creates a new instance of [`ContractScopes`]                         |
//! | [`ContractScopes::all`]                     | Creates a new instance of [`ContractScopes`] with all scopes applied |

/// Struct with methods for listing contract scopes to request for OAuth2
pub struct ContractScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for ContractScopes {
    /// Create a default instance of [`ContractScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl ContractScopes {
    /// Create a new instance of [`ContractScopes`]
    pub fn new() -> Self {
        ContractScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`ContractScopes`] with all scopes applied
    pub fn all() -> Self {
        ContractScopes::new()
    }
}
