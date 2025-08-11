use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an alliance in EVE Online.
///
/// Alliances are player-run organizations made up of multiple corporations.
/// This struct contains the basic information about an alliance as returned by the EVE ESI API.
///
/// # Fields
/// The struct contains identifying information about the alliance, including:
/// - IDs for the alliance's creator (both character and corporation)
/// - The founding date of the alliance
/// - The ID of the alliance's executor corporation if the alliance is not disbanded.
/// - The ID of the alliance's faction if applicable.
/// - Basic identifying information such as name and ticker
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Alliance {
    /// The ID of the corporation that created the alliance.
    pub creator_corporation_id: i32,
    /// The ID of the character that created the alliance.
    pub creator_id: i32,
    /// The date and time when the alliance was founded.
    pub date_founded: DateTime<Utc>,
    /// The ID of the alliance's executor corporation.
    /// If the alliance is disbanded, this will return as None.
    pub executor_corporation_id: Option<i32>,
    /// The ID of the alliance's faction if applicable.
    pub faction_id: Option<i32>,
    /// The name of the alliance.
    pub name: String,
    /// The ticker of the alliance.
    pub ticker: String,
}
