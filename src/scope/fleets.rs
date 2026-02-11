//! # EVE ESI Fleets Scopes
//!
//! This module provides a type-safe way to add fleet-related scopes for OAuth2 to the [`super::ScopeBuilder`]

/// Read access to character fleet information
pub const READ_FLEET: &str = "esi-fleets.read_fleet.v1";
/// Write access to fleet if character holds sufficient fleet roles
pub const WRITE_FLEET: &str = "esi-fleets.write_fleet.v1";

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
        FleetsScopes::new().read_fleet().write_fleet()
    }

    /// Read access to character fleet information
    ///
    /// Adds the `esi-fittings.read_fittings.v1` scope
    pub fn read_fleet(mut self) -> Self {
        self.scopes.push(READ_FLEET.to_string());
        self
    }

    /// Write access to fleet if character holds sufficient fleet roles
    ///
    /// Adds the `esi-fittings.write_fittings.v1` scope
    pub fn write_fleet(mut self) -> Self {
        self.scopes.push(WRITE_FLEET.to_string());
        self
    }
}

#[cfg(test)]
mod fleets_scopes_tests {
    use crate::scope::FleetsScopes;

    /// Tests initializing a default instance of [`FleetsScopes`]
    #[test]
    fn test_fleets_scopes_default() {
        let fleets_scopes = FleetsScopes::default();

        assert_eq!(fleets_scopes.scopes.len(), 0)
    }
}
