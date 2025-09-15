//! Data structures and types for representing characters in EVE Online.
//!
//! This module defines the `Character` & `CharacterAffiliation` structs,
//! which model the core properties of a character & character affiliation in EVE Online.
//!
//! See [ESI API documentation](https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdGet)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::LocationFlag;

/// Represents a character in EVE Online.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Character {
    /// The ID of the alliance the character belongs to, if applicable.
    pub alliance_id: Option<i64>,
    /// The date and time when the character was created.
    pub birthday: DateTime<Utc>,
    /// The ID representing the character's bloodline, which determines cosmetic appearance.
    pub bloodline_id: i64,
    /// The ID of the corporation the character is a member of.
    pub corporation_id: i64,
    /// An optional self-written description of the character.
    pub description: Option<String>,
    /// The ID of the faction the character is associated with, if applicable.
    pub faction_id: Option<i64>,
    /// The gender of the character.
    pub gender: String,
    /// The name of the character.
    pub name: String,
    /// The ID representing the character's race (Amarr, Caldari, Gallente, or Minmatar).
    pub race_id: i64,
    /// The security status of the character, ranging from -10 to +10.
    pub security_status: Option<f64>,
    /// An optional title displayed for the character.
    pub title: Option<String>,
}

/// Represents the affiliations of a character in EVE Online.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersAffiliationPost>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterAffiliation {
    /// The ID of the alliance the character belongs to, if applicable.
    pub alliance_id: Option<i64>,
    /// The unique identifier for this character.
    pub character_id: i64,
    /// The ID of the corporation the character is a member of.
    pub corporation_id: i64,
    /// The ID of the faction the character is associated with, if applicable.
    pub faction_id: Option<i64>,
}

/// Information regarding a character's research agent
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAgentsResearchGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterResearchAgent {
    /// ID of the research agent
    pub agent_id: i64,
    /// Research points accumulated per day
    pub points_per_day: f64,
    /// Remaining research points available to spend
    pub remainder_points: f64,
    /// Specialty skill of the research agent
    pub skill_type_id: i64,
    /// Date and time the research agent was started
    pub started_at: DateTime<Utc>,
}

/// Information regarding a character's blueprints
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdBlueprintsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Blueprint {
    /// Unique ID for the item
    item_id: i64,
    /// Type of the location_id
    location_flag: LocationFlag,
    /// References a station, ship, or an item_id if the blueprint is within a container. If the return
    /// value is an item_id then the [Character AssetList API](https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdAssets)
    /// must be queried to find the container using the given item_id to determine the correct location of the blueprint.
    location_id: i64,
    /// Material efficiency level of the blueprint
    material_efficiency: i64,
    /// A range of numbers with a minimum of -2 and no maximum value where -1 is an original and -2 is a copy.
    /// It can be a positive integer if it is a stack of blueprint originals fresh from the market (e.g. no activities performed on them yet).
    quantity: i64,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original.
    runs: i64,
    /// Time Efficiency Level of the blueprint.
    time_efficiency: i64,
    /// Represents the type of blueprint
    type_id: i64,
}
