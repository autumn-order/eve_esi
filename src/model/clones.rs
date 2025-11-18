//! # EVE ESI Clone Models
//!
//! Provides clone-related structs for EVE Online
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>
//!
//! ## Models
//! - [`CharacterCloneHomeLocation`]: A character's home station
//! - [`CharacterJumpClone`]: An entry for a character's jump clones
//! - [`CharacterClones`]: Character's home station, list of jump clones, and info on last home station change and last clone jump

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::clones::CharacterCloneLocationType;

/// A character's home station
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdClonesGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterCloneHomeLocation {
    /// Location ID of the station or structure
    pub location_id: i64,
    /// Type of location where the clone is located
    pub location_type: CharacterCloneLocationType,
}

/// An entry for a character's jump clones
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdClonesGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterJumpClone {
    /// List of type IDs for clone's implants
    ///
    /// Note: if clone has no implants, the Vec will contain a None value
    pub implants: Vec<Option<i64>>,
    /// Unique ID of the jump clone
    pub jump_clone_id: i64,
    /// Location ID of the station of structure
    pub location_id: i64,
    /// Type of location where the clone is located
    pub location_type: CharacterCloneLocationType,
    /// Name of the clone if it is set
    pub name: Option<String>,
}

/// Character's home station, list of jump clones, and info on last home station change and last clone jump
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdClonesGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterClones {
    /// The character's home station
    pub home_location: Option<CharacterCloneHomeLocation>,
    /// List of character's jump clones
    pub jump_clones: Vec<CharacterJumpClone>,
    /// The last time that character jump cloned
    pub last_clone_jump_date: DateTime<Utc>,
    /// Last time character changed their home station
    pub last_station_change_date: DateTime<Utc>,
}
