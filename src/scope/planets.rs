//! # EVE ESI Planets Scopes
//!
//! This module provides a type-safe way to add planet-related scopes for OAuth2 to the [`super::ScopeBuilder`]

/// Read access to character's planetary interaction
pub const MANAGE_PLANETS: &str = "esi-planets.manage_planets.v1";
/// Read access to corporation-owned customs offices
pub const READ_CUSTOMS_OFFICES: &str = "esi-planets.read_customs_offices.v1";

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
        PlanetsScopes::new().manage_planets().read_customs_offices()
    }

    /// Read access to character's planetary interaction
    ///
    /// Adds the `esi-planets.manage_planets.v1` scope
    pub fn manage_planets(mut self) -> Self {
        self.scopes.push(MANAGE_PLANETS.to_string());
        self
    }

    /// Read access to corporation-owned customs offices
    ///
    /// Adds the `esi-planets.read_customs_offices.v1` scope
    pub fn read_customs_offices(mut self) -> Self {
        self.scopes.push(READ_CUSTOMS_OFFICES.to_string());
        self
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
