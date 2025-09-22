//! # EVE ESI User Interface Scopes
//!
//! This module provides a type-safe way to add user interface-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                           | Description                                                               |
//! | ------------------------------------------------ | ------------------------------------------------------------------------- |
//! | [`UserInterfaceScopes::new`]                     | Creates a new instance of [`UserInterfaceScopes`]                         |
//! | [`UserInterfaceScopes::all`]                     | Creates a new instance of [`UserInterfaceScopes`] with all scopes applied |

/// Struct with methods for listing user interface scopes to request for OAuth2
pub struct UserInterfaceScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for UserInterfaceScopes {
    /// Create a default instance of [`UserInterfaceScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl UserInterfaceScopes {
    /// Create a new instance of [`UserInterfaceScopes`]
    pub fn new() -> Self {
        UserInterfaceScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`UserInterfaceScopes`] with all scopes applied
    pub fn all() -> Self {
        UserInterfaceScopes::new()
    }
}
