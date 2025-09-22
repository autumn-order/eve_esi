//! # EVE ESI Contracts Scopes
//!
//! This module provides a type-safe way to add contract-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                       | Description                                                           |
//! | -------------------------------------------- | --------------------------------------------------------------------- |
//! | [`ContractsScopes::new`]                     | Creates a new instance of [`ContractsScopes`]                         |
//! | [`ContractsScopes::all`]                     | Creates a new instance of [`ContractsScopes`] with all scopes applied |

/// Read access to character contracts
pub const READ_CHARACTER_CONTRACTS: &str = "esi-contracts.read_character_contracts.v1";
/// Read access to corporation contracts
pub const READ_CORPORATION_CONTRACTS: &str = "esi-contracts.read_corporation_contracts.v1";

/// Struct with methods for listing contract scopes to request for OAuth2
pub struct ContractsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for ContractsScopes {
    /// Create a default instance of [`ContractsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl ContractsScopes {
    /// Create a new instance of [`ContractsScopes`]
    pub fn new() -> Self {
        ContractsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`ContractsScopes`] with all scopes applied
    pub fn all() -> Self {
        ContractsScopes::new()
    }
}

#[cfg(test)]
mod contracts_scopes_tests {
    use crate::scope::ContractsScopes;

    /// Tests initializing a default instance of [`ContractsScopes`]
    #[test]
    fn test_contracts_scopes_default() {
        let contracts_scopes = ContractsScopes::default();

        assert_eq!(contracts_scopes.scopes.len(), 0)
    }
}
