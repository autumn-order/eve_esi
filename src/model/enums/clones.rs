//! # EVE ESI Clone Enums
//!
//! Provides clone-related enums for EVE Online
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>
//!
//! ## Enums
//! - [`CharacterCloneLocationType`]: The location type of a character's clone

use serde::{Deserialize, Serialize};

/// The location type of a character's clone
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdClonesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CharacterCloneLocationType {
    /// Clone is located in a station
    #[serde(rename = "station")]
    Station,
    /// Clone is located in a structure
    #[serde(rename = "structure")]
    Structure,
}
