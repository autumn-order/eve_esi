//! # EVE ESI Planets Scopes
//!
//! This module provides a type-safe way to add planet-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                        |
//! | ----------------------------------------- | ------------------------------------------------------------------ |
//! | [`PlanetScopes::new`]                     | Creates a new instance of [`PlanetScopes`]                         |
//! | [`PlanetScopes::all`]                     | Creates a new instance of [`PlanetScopes`] with all scopes applied |

/// Struct with methods for listing planet scopes to request for OAuth2
pub struct PlanetScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for PlanetScopes {
    /// Create a default instance of [`PlanetScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl PlanetScopes {
    /// Create a new instance of [`PlanetScopes`]
    pub fn new() -> Self {
        PlanetScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`PlanetScopes`] with all scopes applied
    pub fn all() -> Self {
        PlanetScopes::new()
    }
}
