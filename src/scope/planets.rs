//! # EVE ESI Planets Scopes
//!
//! This module provides a type-safe way to add planet-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                     | Description                                                         |
//! | ------------------------------------------ | ------------------------------------------------------------------- |
//! | [`PlanetsScopes::new`]                     | Creates a new instance of [`PlanetsScopes`]                         |
//! | [`PlanetsScopes::all`]                     | Creates a new instance of [`PlanetsScopes`] with all scopes applied |

/// Struct with methods for listing planet scopes to request for OAuth2
pub struct PlanetsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for PlanetsScopes {
    /// Create a default instance of [`PlanetsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl PlanetsScopes {
    /// Create a new instance of [`PlanetsScopes`]
    pub fn new() -> Self {
        PlanetsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`PlanetsScopes`] with all scopes applied
    pub fn all() -> Self {
        PlanetsScopes::new()
    }
}

#[cfg(test)]
mod planets_scopes_tests {
    use crate::scope::PlanetsScopes;

    /// Tests initializing a default instance of [`PlanetsScopes`]
    #[test]
    fn test_planets_scopes_default() {
        let planets_scopes = PlanetsScopes::default();

        assert_eq!(planets_scopes.scopes.len(), 0)
    }
}
