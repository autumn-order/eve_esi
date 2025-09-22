//! # EVE ESI Calendar Scopes
//!
//! This module provides a type-safe way to add calendar-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                      | Description                                                          |
//! | ------------------------------------------- | -------------------------------------------------------------------- |
//! | [`CalendarScopes::new`]                     | Creates a new instance of [`CalendarScopes`]                         |
//! | [`CalendarScopes::all`]                     | Creates a new instance of [`CalendarScopes`] with all scopes applied |

///Access to read calendar events
pub const READ_CALENDAR_EVENTS: &str = "esi-calendar.read_calendar_events.v1";
/// Access to respond to calendar events on behalf of the character
pub const RESPOND_CALENDAR_EVENTS: &str = "esi-calendar.respond_calendar_events.v1";

/// Struct with methods for listing calendar scopes to request for OAuth2
pub struct CalendarScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for CalendarScopes {
    /// Create a default instance of [`CalendarScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl CalendarScopes {
    /// Create a new instance of [`CalendarScopes`]
    pub fn new() -> Self {
        CalendarScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`CalendarScopes`] with all scopes applied
    pub fn all() -> Self {
        CalendarScopes::new()
    }
}

#[cfg(test)]
mod calendar_scopes_tests {
    use crate::scope::CalendarScopes;

    /// Tests initializing a default instance of [`CalendarScopes`]
    #[test]
    fn test_calendar_scopes_default() {
        let calendar_scopes = CalendarScopes::default();

        assert_eq!(calendar_scopes.scopes.len(), 0)
    }
}
