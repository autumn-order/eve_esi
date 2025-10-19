//! # EVE ESI Universe Models
//!
//! Provides universe-related structs for EVE Online
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>
//!
//! ## Models
//!
//! | Model                  | Description                                                                 |
//! | ---------------------- | --------------------------------------------------------------------------- |
//! | [`Faction`]            | Represents an NPC faction in EVE Online                                     |

use serde::{Deserialize, Serialize};

/// Represents an NPC faction in EVE Online
///
/// # Documentation
///- <https://developers.eveonline.com/api-explorer#/schemas/UniverseFactionsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Faction {
    /// Primary corporation ID for the faction if applicable
    corporation_d: Option<i64>,
    /// Description for the faction
    description: String,
    /// Unique ID of the faction
    faction_id: i64,
    /// ???
    is_unique: bool,
    /// The faction warfare militia corporation if applicable
    militia_corporation_id: Option<i64>,
    /// The name of the faction
    name: String,
    /// The size of the faction
    size_factor: f64,
    /// The unique ID of the faction's home system if applicable
    solar_system_id: Option<i64>,
    /// The amount of NPC stations the faction owns
    station_count: i64,
    /// The amount of systems the NPC faction has stations in
    station_system_count: i64,
}
