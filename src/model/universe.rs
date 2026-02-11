//! # EVE ESI Universe Models
//!
//! Provides universe-related structs for EVE Online

use serde::{Deserialize, Serialize};

/// Represents an NPC faction in EVE Online
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/UniverseFactionsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Faction {
    /// Primary corporation ID for the faction if applicable
    pub corporation_id: Option<i64>,
    /// Description for the faction
    pub description: String,
    /// Unique ID of the faction
    pub faction_id: i64,
    pub is_unique: bool,
    /// The faction warfare militia corporation if applicable
    pub militia_corporation_id: Option<i64>,
    /// The name of the faction
    pub name: String,
    /// The size of the faction
    pub size_factor: f64,
    /// The unique ID of the faction's home system if applicable
    pub solar_system_id: Option<i64>,
    /// The amount of NPC stations the faction owns
    pub station_count: i64,
    /// The amount of systems the NPC faction has stations in
    pub station_system_count: i64,
}
