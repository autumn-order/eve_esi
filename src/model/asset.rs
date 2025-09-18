//! # EVE ESI Asset Models
//!
//! Provides structs representing assets within EVE Online such as blueprints
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>

use serde::{Deserialize, Serialize};

use super::enums::asset::LocationFlag;

/// Information regarding a character's blueprints
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdBlueprintsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Blueprint {
    /// Unique ID for the item
    pub item_id: i64,
    /// Type of the location_id
    pub location_flag: LocationFlag,
    /// References a station, ship, or an item_id if the blueprint is within a container. If the return
    /// value is an item_id then the [Character AssetList API](https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdAssets)
    /// must be queried to find the container using the given item_id to determine the correct location of the blueprint.
    pub location_id: i64,
    /// Material efficiency level of the blueprint
    pub material_efficiency: i64,
    /// A range of numbers with a minimum of -2 and no maximum value where -1 is an original and -2 is a copy.
    /// It can be a positive integer if it is a stack of blueprint originals fresh from the market (e.g. no activities performed on them yet).
    pub quantity: i64,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original.
    pub runs: i64,
    /// Time Efficiency Level of the blueprint.
    pub time_efficiency: i64,
    /// Represents the type of blueprint
    pub type_id: i64,
}
