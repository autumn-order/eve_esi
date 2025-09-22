//! # EVE ESI Killmails Scopes
//!
//! This module provides a type-safe way to add killmail-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                          |
//! | ----------------------------------------- | -------------------------------------------------------------------- |
//! | [`KillmailScopes::new`]                   | Creates a new instance of [`KillmailScopes`]                         |
//! | [`KillmailScopes::all`]                   | Creates a new instance of [`KillmailScopes`] with all scopes applied |

/// Struct with methods for listing killmail scopes to request for OAuth2
pub struct KillmailScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for KillmailScopes {
    /// Create a default instance of [`KillmailScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl KillmailScopes {
    /// Create a new instance of [`KillmailScopes`]
    pub fn new() -> Self {
        KillmailScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`KillmailScopes`] with all scopes applied
    pub fn all() -> Self {
        KillmailScopes::new()
    }
}
