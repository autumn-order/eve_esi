//! # EVE ESI Calendar Models
//!
//! Provides calendar-related structs for EVE Online
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::calendar::CalendarEventResponse;

/// A calendar event in EVE Online
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdCalendarGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CalendarEvent {
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
