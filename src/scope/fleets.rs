//! # EVE ESI Fleets Scopes
//!
//! This module provides a type-safe way to add fleet-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                   | Description                                                       |
//! | ---------------------------------------- | ----------------------------------------------------------------- |
//! | [`FleetScopes::new`]                     | Creates a new instance of [`FleetScopes`]                         |
//! | [`FleetScopes::all`]                     | Creates a new instance of [`FleetScopes`] with all scopes applied |

/// Struct with methods for listing fleet scopes to request for OAuth2
pub struct FleetScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for FleetScopes {
    /// Create a default instance of [`FleetScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl FleetScopes {
    /// Create a new instance of [`FleetScopes`]
    pub fn new() -> Self {
        FleetScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`FleetScopes`] with all scopes applied
    pub fn all() -> Self {
        FleetScopes::new()
    }
}
