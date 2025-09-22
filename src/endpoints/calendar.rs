//! # EVE ESI Calendar Endpoints
//!
//! This module provides the [`CalendarEndpoints`] struct and associated methods for accessing
//! calendar-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (0)
//! ### Public (0)
//!
//! | Endpoint | Description |
//! | -------- | ----------- |
//! |          |             |
//!
//! ### Authenticated (0)
//!
//! | Endpoint | Description |
//! | -------- | ----------- |
//! |          |             |

use crate::{model::calendar::CalendarEvent, scope::CalendarScopes, Client, Error, ScopeBuilder};

/// Provides methods for accessing calendar-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct CalendarEndpoints<'a> {
    client: &'a Client,
}

impl<'a> CalendarEndpoints<'a> {
    /// Creates a new instance of [`CalendarEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Get list of summaries for the last 50 calendar events for provided character ID
        ///
        /// Note: does not currently support the optional parameter `from_event` specified
        /// in ESI docs to receive summary for a particular event ID. This will be implemented
        /// in a later refactor of the underlying `define_endpoint!` macro to handle optional
        /// params.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdCalendar>
        ///
        /// # Required Scopes
        /// - [`CalendarScopes::read_calendar_events`](crate::scope::CalendarScopes::read_calendar_events):
        ///   `esi-assets.read_calendar_events.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve calendar event summaries for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`CalendarEvent`]>: list of summaries for the last 50 calendar events for provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get list_calendar_event_summaries(
            access_token: &str,
            character_id: i64,
        ) -> Result<Vec<CalendarEvent>, Error>
        url = "{}/characters/{}/calendar";
        label = "calendar events";
        required_scopes = ScopeBuilder::new()
            .calendar(CalendarScopes::new().read_calendar_events())
            .build();
    }
}
