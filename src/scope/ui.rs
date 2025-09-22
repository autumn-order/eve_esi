//! # EVE ESI User Interface Scopes
//!
//! This module provides a type-safe way to add user interface-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                           | Description                                                    |
//! | ------------------------------------------------ | -------------------------------------------------------------- |
//! | [`UiScopes::new`]                                | Creates a new instance of [`UiScopes`]                         |
//! | [`UiScopes::all`]                                | Creates a new instance of [`UiScopes`] with all scopes applied |

pub const OPEN_WINDOW: &str = "esi-ui.open_window.v1";
pub const WRITE_WAYPOINT: &str = "esi-ui.write_waypoint.v1";

/// Struct with methods for listing user interface scopes to request for OAuth2
pub struct UiScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for UiScopes {
    /// Create a default instance of [`UiScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl UiScopes {
    /// Create a new instance of [`UiScopes`]
    pub fn new() -> Self {
        UiScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`UiScopes`] with all scopes applied
    pub fn all() -> Self {
        UiScopes::new()
    }
}

#[cfg(test)]
mod ui_scopes_tests {
    use crate::scope::UiScopes;

    /// Tests initializing a default instance of [`UiScopes`]
    #[test]
    fn test_ui_scopes_default() {
        let ui_scopes = UiScopes::default();

        assert_eq!(ui_scopes.scopes.len(), 0)
    }
}
