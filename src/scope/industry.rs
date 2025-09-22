//! # EVE ESI Industry Scopes
//!
//! This module provides a type-safe way to add industry-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                      | Description                                                          |
//! | ------------------------------------------- | -------------------------------------------------------------------- |
//! | [`IndustryScopes::new`]                     | Creates a new instance of [`IndustryScopes`]                         |
//! | [`IndustryScopes::all`]                     | Creates a new instance of [`IndustryScopes`] with all scopes applied |

/// Struct with methods for listing industry scopes to request for OAuth2
pub struct IndustryScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for IndustryScopes {
    /// Create a default instance of [`IndustryScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl IndustryScopes {
    /// Create a new instance of [`IndustryScopes`]
    pub fn new() -> Self {
        IndustryScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`IndustryScopes`] with all scopes applied
    pub fn all() -> Self {
        IndustryScopes::new()
    }
}
