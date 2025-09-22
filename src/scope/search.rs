//! # EVE ESI Search Scopes
//!
//! This module provides a type-safe way to add search-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                        |
//! | ----------------------------------------- | ------------------------------------------------------------------ |
//! | [`SearchScopes::new`]                     | Creates a new instance of [`SearchScopes`]                         |
//! | [`SearchScopes::all`]                     | Creates a new instance of [`SearchScopes`] with all scopes applied |

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
        SearchScopes::new()
    }
}
