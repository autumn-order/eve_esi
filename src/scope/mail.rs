//! # EVE ESI Mail Scopes
//!
//! This module provides a type-safe way to add mail-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                      |
//! | ----------------------------------------- | ---------------------------------------------------------------- |
//! | [`MailScopes::new`]                       | Creates a new instance of [`MailScopes`]                         |
//! | [`MailScopes::all`]                       | Creates a new instance of [`MailScopes`] with all scopes applied |

/// Access to organize character's mail
pub const ORGANIZE_MAIL: &str = "esi-mail.organize_mail.v1";
/// Read access to character's eve mails
pub const READ_MAIL: &str = "esi-mail.read_mail.v1";
/// Write access to send eve mails on behalf of character
pub const SEND_MAIL: &str = "esi-mail.send_mail.v1";

/// Struct with methods for listing mail scopes to request for OAuth2
pub struct MailScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for MailScopes {
    /// Create a default instance of [`MailScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl MailScopes {
    /// Create a new instance of [`MailScopes`]
    pub fn new() -> Self {
        MailScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`MailScopes`] with all scopes applied
    pub fn all() -> Self {
        MailScopes::new()
    }
}

#[cfg(test)]
mod mail_scopes_tests {
    use crate::scope::MailScopes;

    /// Tests initializing a default instance of [`MailScopes`]
    #[test]
    fn test_mail_scopes_default() {
        let mail_scopes = MailScopes::default();

        assert_eq!(mail_scopes.scopes.len(), 0)
    }
}
