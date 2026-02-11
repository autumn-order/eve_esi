//! # EVE ESI Search Scopes
//!
//! This module provides a type-safe way to add search-related scopes for OAuth2 to the [`super::ScopeBuilder`]

/// Access to search for structures character has access to
pub const SEARCH_STRUCTURES: &str = "esi-search.search_structures.v1";

/// Struct with methods for listing search scopes to request for OAuth2
pub struct SearchScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for SearchScopes {
    /// Create a default instance of [`SearchScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl SearchScopes {
    /// Create a new instance of [`SearchScopes`]
    pub fn new() -> Self {
        SearchScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`SearchScopes`] with all scopes applied
    pub fn all() -> Self {
        SearchScopes::new().search_structures()
    }

    /// Access to search for structures character has access to
    ///
    /// Adds the `esi-search.search_structures.v1` scope
    pub fn search_structures(mut self) -> Self {
        self.scopes.push(SEARCH_STRUCTURES.to_string());
        self
    }
}

#[cfg(test)]
mod search_scopes_tests {
    use crate::scope::SearchScopes;

    /// Tests initializing a default instance of [`SearchScopes`]
    #[test]
    fn test_search_scopes_default() {
        let search_scopes = SearchScopes::default();

        assert_eq!(search_scopes.scopes.len(), 0)
    }
}
