//! Data structures and types for representing alliances in EVE Online.
//!
//! This module defines the `Alliance` struct, which models the core properties of an alliance in EVE Online.
//!
//! See [ESI API documentation](https://developers.eveonline.com/api-explorer)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an alliance in EVE Online.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/AlliancesAllianceIdGet>
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

/// Reoresents the 128x128 & 64x64 icon URLs for an alliance
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/AlliancesAllianceIdIconsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AllianceIcons {
    /// 128x128 icon URL for an alliance
    pub px128x128: String,
    /// 64x64 icon URL for an alliance
    pub px64x64: String,
}
