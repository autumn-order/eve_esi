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
//! ## Endpoints (4)
//!
//! ### Authenticated (4)
//!
//! | Endpoint                                             | Description                                                                                       |
//! | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
//! | [`CalendarEndpoints::list_calendar_event_summaries`] | Get list of summaries for the last 50 calendar events for provided character ID                   |
//! | [`CalendarEndpoints::get_an_event`]                  | Get all information for the provided calendar event ID                                            |
//! | [`CalendarEndpoints::respond_to_an_event`]           | Respond to a calendar event on behalf of the provided character ID                                |
//! | [`CalendarEndpoints::get_attendees`]                 | Get list of calendar event attendee character IDs & responses for the event ID                    |

use crate::{
    esi::EsiRequest,
    model::{
        calendar::{CalendarEvent, CalendarEventAttendee, CalendarEventSummary},
        enums::calendar::PutCalendarEventResponse,
    },
    scope::CalendarScopes,
    Client, Error, ScopeBuilder,
};
use reqwest::Method;

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

    define_esi_endpoint! {
        /// Get list of summaries for the last 50 calendar events for provided character ID
        ///
        /// Note: does not currently support the optional parameter `from_event` specified
        /// in ESI docs to receive summary for a particular event ID. This will be implemented
        /// in a later refactor of the underlying `define_endpoint!` macro to handle optional
        /// params.
        ///
        /// For now, the [`CalendarEndpoints::get_an_event`] method will work as a sufficient alternative.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdCalendar>
        ///
        /// # Required Scopes
        /// - [`CalendarScopes::read_calendar_events`](crate::scope::CalendarScopes::read_calendar_events):
        ///   `esi-calendar.read_calendar_events.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve calendar event summaries for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of calendar event summaries
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn list_calendar_event_summaries(
            access_token: &str,
            character_id: i64
        ) -> Result<EsiRequest<Vec<CalendarEventSummary>>, Error>
        method = Method::GET;
        url = "{}/characters/{}/calendar";
        required_scopes = ScopeBuilder::new()
            .calendar(CalendarScopes::new().read_calendar_events())
            .build();
    }

    define_esi_endpoint! {
        /// Get all information for the provided calendar event ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdCalendarEventId>
        ///
        /// # Required Scopes
        /// - [`CalendarScopes::read_calendar_events`](crate::scope::CalendarScopes::read_calendar_events):
        ///   `esi-calendar.read_calendar_events.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve calendar event for.
        /// - `event_id`      (`i64`): The ID of the calendar event to retrieve information for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for calendar event information
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_an_event(
            access_token: &str,
            character_id: i64,
            event_id: i64
        ) -> Result<EsiRequest<CalendarEvent>, Error>
        method = Method::GET;
        url = "{}/characters/{}/calendar/{}";
        required_scopes = ScopeBuilder::new()
            .calendar(CalendarScopes::new().read_calendar_events())
            .build();
    }

    define_esi_endpoint! {
        /// Respond to a calendar event on behalf of the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/PutCharactersCharacterIdCalendarEventId>
        ///
        /// # Required Scopes
        /// - [`CalendarScopes::respond_calendar_events`](crate::scope::CalendarScopes::respond_calendar_events):
        ///   `esi-calendar.respond_calendar_events.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to respond to the event on behalf of.
        /// - `event_id`      (`i64`): The ID of the calendar event to respond to
        /// - `event_response` ([`PutCalendarEventResponse`]): The response to send for the character
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for the response action
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn respond_to_an_event(
            access_token: &str,
            character_id: i64,
            event_id: i64
        ) -> Result<EsiRequest<()>, Error>
        method = Method::PUT;
        url = "{}/characters/{}/calendar/{}";
        required_scopes = ScopeBuilder::new()
            .calendar(CalendarScopes::new().respond_calendar_events())
            .build();
        body = event_response: PutCalendarEventResponse;
    }

    define_esi_endpoint! {
        /// Get list of calendar event attendee character IDs & responses for the event ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdCalendar>
        ///
        /// # Required Scopes
        /// - [`CalendarScopes::read_calendar_events`](crate::scope::CalendarScopes::read_calendar_events):
        ///   `esi-calendar.read_calendar_events.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve calendar event attendees for.
        /// - `event_id`      (`i64`): The ID of the calendar event to retrieve attendees for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of calendar event attendees
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_attendees(
            access_token: &str,
            character_id: i64,
            event_id: i64
        ) -> Result<EsiRequest<Vec<CalendarEventAttendee>>, Error>
        method = Method::GET;
        url = "{}/characters/{}/calendar/{}/attendees";
        required_scopes = ScopeBuilder::new()
            .calendar(CalendarScopes::new().read_calendar_events())
            .build();
    }
}
