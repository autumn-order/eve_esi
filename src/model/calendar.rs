//! # EVE ESI Calendar Models
//!
//! Provides calendar-related structs for EVE Online
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::calendar::{CalendarEventOwnerType, CalendarEventResponse};

/// A calendar event in EVE Online
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdCalendarGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CalendarEventSummary {
    /// Timestamp of the event
    event_date: DateTime<Utc>,
    /// Unique ID of the event
    event_id: i64,
    /// Character's response to the event
    event_response: CalendarEventResponse,
    /// Importance of the event
    importance: i64,
    /// Name of the event
    title: String,
}

/// A calendar event in EVE Online
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdCalendarEventIdGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CalendarEvent {
    /// Timestamp of the event
    date: DateTime<Utc>,
    /// Length of the event in minutes
    duration: i64,
    /// Unique ID of the event
    event_id: i64,
    /// Importance of the event
    importance: i64,
    /// Unique ID of the event owner
    owner_id: i64,
    /// Name of the event owner
    owner_name: String,
    /// The type of calendar event owner
    owner_type: CalendarEventOwnerType,
    /// Character's response to the event
    // Maybe this is CalendarEventResponse enum but ESI documentation doesn't show an enum here
    response: String,
    /// Description of the event
    text: String,
    /// Name of the event
    title: String,
}

/// An entry for a calendar event attendee character ID & their response to the event
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdCalendarEventIdAttendeesGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CalendarEventAttendee {
    /// Unique ID of the character
    character_id: i64,
    /// Character's response to the event
    event_response: CalendarEventResponse,
}
