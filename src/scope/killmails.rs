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

/// Read access to corporation killmails
pub const READ_CORPORATION_KILLMAILS: &str = "esi-killmails.read_corporation_killmails.v1";
/// Read access to character killmails
pub const READ_KILLMAILS: &str = "esi-killmails.read_killmails.v1";

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

#[cfg(test)]
mod killmails_scopes_tests {
    use crate::scope::KillmailsScopes;

    /// Tests initializing a default instance of [`KillmailsScopes`]
    #[test]
    fn test_killmails_scopes_default() {
        let killmails_scopes = KillmailsScopes::default();

        assert_eq!(killmails_scopes.scopes.len(), 0)
    }
}
