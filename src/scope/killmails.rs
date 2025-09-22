//! # EVE ESI Killmails Scopes
//!
//! This module provides a type-safe way to add killmail-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                     | Description                                                           |
//! | ------------------------------------------ | --------------------------------------------------------------------- |
//! | [`KillmailsScopes::new`]                   | Creates a new instance of [`KillmailsScopes`]                         |
//! | [`KillmailsScopes::all`]                   | Creates a new instance of [`KillmailsScopes`] with all scopes applied |

/// Struct with methods for listing killmail scopes to request for OAuth2
pub struct KillmailsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for KillmailsScopes {
    /// Create a default instance of [`KillmailsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl KillmailsScopes {
    /// Create a new instance of [`KillmailsScopes`]
    pub fn new() -> Self {
        KillmailsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`KillmailsScopes`] with all scopes applied
    pub fn all() -> Self {
        KillmailsScopes::new()
    }
}
