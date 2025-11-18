//! # EVE ESI Character Enums
//!
//! Provides enums related to characters in EVE Online
//!
//! ## Enums
//! - [`CharacterMedalStatus`]: Represents the visbility status of a character's medal

use serde::{Deserialize, Serialize};

/// Represents the visbility status of a character's medal
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdMedalsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CharacterMedalStatus {
    /// Medal visibility is set to public
    #[serde(rename = "public")]
    Public,
    /// Medal visibility is set to private
    #[serde(rename = "private")]
    Private,
}
