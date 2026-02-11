//! # EVE ESI Clones Scopes
//!
//! This module provides a type-safe way to add clone-related scopes for OAuth2 to the [`super::ScopeBuilder`]

/// Access to read information on character's clones
pub const READ_CLONES: &str = "esi-clones.read_clones.v1";
/// Access to read character's implants
pub const READ_IMPLANTS: &str = "esi-clones.read_implants.v1";

/// Struct with methods for listing clone scopes to request for OAuth2
pub struct ClonesScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for ClonesScopes {
    /// Create a default instance of [`ClonesScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl ClonesScopes {
    /// Create a new instance of [`ClonesScopes`]
    pub fn new() -> Self {
        ClonesScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`ClonesScopes`] with all scopes applied
    pub fn all() -> Self {
        ClonesScopes::new().read_clones().read_implants()
    }

    /// Access to read information on character's clones
    ///
    /// Adds the `esi-clones.read_clones.v1` scope
    pub fn read_clones(mut self) -> Self {
        self.scopes.push(READ_CLONES.to_string());
        self
    }

    /// Access to read character's implants
    ///
    /// Adds the `esi-clones.read_implants.v1` scope
    pub fn read_implants(mut self) -> Self {
        self.scopes.push(READ_IMPLANTS.to_string());
        self
    }
}

#[cfg(test)]
mod clones_scopes_tests {
    /// Access to read a character's contacts
    use crate::scope::ClonesScopes;

    /// Tests initializing a default instance of [`ClonesScopes`]
    #[test]
    fn test_clones_scopes_default() {
        let clones_scopes = ClonesScopes::default();

        assert_eq!(clones_scopes.scopes.len(), 0)
    }
}
