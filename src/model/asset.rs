//! # EVE ESI Asset Models
//!
//! Provides structs representing assets within EVE Online such as blueprints
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>

use serde::{Deserialize, Serialize};

use crate::model::enums::asset::LocationType;

use super::enums::asset::LocationFlag;

/// Information regarding a character's blueprints
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdBlueprintsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

/// An asset in EVE Online
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAssetsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Asset {
    /// If item is a blueprint, bool indicating whether or not it is a copy
    is_blueprint_copy: Option<bool>,
    /// If item is stackable or not
    is_singleton: bool,
    /// Unique ID of the item
    item_id: i64,
    /// Flag indicating the location of the item
    location_flag: LocationFlag,
    /// ID of the item's location
    location_id: i64,
    /// The type of location ID
    location_type: LocationType,
    /// The quantity of the item
    quantity: i64,
    /// Type ID of the item
    type_id: i64,
}

/// The coordinates of where an item is located in space
///
/// Coordinates will be (0,0,0) if in a station or hangar
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAssetsLocationsPost>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssetLocationPosition {
    /// X coordinate of the item in space
    x: f64,
    /// Y coordinate of the item in space
    y: f64,
    /// Z coordinate of the item in space
    z: f64,
}

/// Where an asset is located in space in EVE Online
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAssetsLocationsPost>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssetLocation {
    /// Unique ID of the item
    item_id: i64,
    /// Item coordinates in space, coordinates will be (0,0,0) if in a station or hangar
    position: AssetLocationPosition,
}

/// Repesents an asset's item ID and name of the item
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAssetsNamesPost>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssetName {
    /// Unique ID of the item
    item_id: i64,
    /// Name of the item
    name: String,
}
