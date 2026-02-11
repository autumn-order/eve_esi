//! # EVE ESI Fittings Scopes
//!
//! This module provides a type-safe way to add fitting-related scopes for OAuth2 to the [`super::ScopeBuilder`]

/// Read access to character fittings
pub const READ_FITTINGS: &str = "esi-fittings.read_fittings.v1";
/// Write access to character fittings
pub const WRITE_FITTINGS: &str = "esi-fittings.write_fittings.v1";

/// Struct with methods for listing fitting scopes to request for OAuth2
pub struct FittingsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for FittingsScopes {
    /// Create a default instance of [`FittingsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl FittingsScopes {
    /// Create a new instance of [`FittingsScopes`]
    pub fn new() -> Self {
        FittingsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`FittingsScopes`] with all scopes applied
    pub fn all() -> Self {
        FittingsScopes::new().read_fittings().write_fittings()
    }

    /// Read access to character fittings
    ///
    /// Adds the `esi-fittings.read_fittings.v1` scope
    pub fn read_fittings(mut self) -> Self {
        self.scopes.push(READ_FITTINGS.to_string());
        self
    }

    /// Write access to character fittings
    ///
    /// Adds the `esi-fittings.write_fittings.v1` scope
    pub fn write_fittings(mut self) -> Self {
        self.scopes.push(WRITE_FITTINGS.to_string());
        self
    }
}

#[cfg(test)]
mod fittings_scopes_tests {
    use crate::scope::FittingsScopes;

    /// Tests initializing a default instance of [`FittingsScopes`]
    #[test]
    fn test_fittings_scopes_default() {
        let fittings_scopes = FittingsScopes::default();

        assert_eq!(fittings_scopes.scopes.len(), 0)
    }
}
