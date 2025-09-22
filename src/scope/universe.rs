//! # EVE ESI Universe Scopes
//!
//! This module provides a type-safe way to add universe-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                      | Description                                                          |
//! | ------------------------------------------- | -------------------------------------------------------------------- |
//! | [`UniverseScopes::new`]                     | Creates a new instance of [`UniverseScopes`]                         |
//! | [`UniverseScopes::all`]                     | Creates a new instance of [`UniverseScopes`] with all scopes applied |

/// Read access to information on structures character has access to
pub const READ_STRUCTURES: &str = "esi-universe.read_structures.v1";

/// Struct with methods for listing universe scopes to request for OAuth2
pub struct UniverseScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for UniverseScopes {
    /// Create a default instance of [`UniverseScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl UniverseScopes {
    /// Create a new instance of [`UniverseScopes`]
    pub fn new() -> Self {
        UniverseScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`UniverseScopes`] with all scopes applied
    pub fn all() -> Self {
        UniverseScopes::new()
    }
}

#[cfg(test)]
mod universe_scopes_tests {
    use crate::scope::UniverseScopes;

    /// Tests initializing a default instance of [`UniverseScopes`]
    #[test]
    fn test_universe_scopes_default() {
        let universe_scopes = UniverseScopes::default();

        assert_eq!(universe_scopes.scopes.len(), 0)
    }
}
