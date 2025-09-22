//! # EVE ESI Calendar Enums
//!
//! Provides enums related to calendars in EVE Online
//!
//! ## Enums
//! - [`CalendarEventResponse`]: The type of location for an asset's location ID

use serde::{Deserialize, Serialize};

/// Represents a character's response to a calendar event
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdCalendar>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CalendarEventResponse {
    /// Character declined calendar event
    #[serde(rename = "declined")]
    Declined,
    /// Character has not responded to calendar event
    #[serde(rename = "not_responded")]
    NotResponded,
    /// Character accepted calendar event
    #[serde(rename = "accepted")]
    Accepted,
    /// Character's calendar event attendance is tentative
    #[serde(rename = "tentative")]
    Tentative,
}
