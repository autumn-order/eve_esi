//! # EVE ESI Character Enums
//!
//! Provides enums related to characters in EVE Online
//!
//! ## Enums
//! - [`CharacterMedalStatus`]: Represents the visbility status of a character's medal
//! - [`CharacterStandingType`]: The type of character standing entry (Agent, NpcCorp, or Faction)

use serde::{Deserialize, Serialize};

/// Represents the visbility status of a character's medal
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdMedalsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CharacterMedalStatus {
    /// Medal visibility is set to public
    #[serde(rename = "public")]
    Public,
    /// Medal visibility is set to private
    #[serde(rename = "private")]
    Private,
}

/// The type of character standing entry (Agent, NpcCorp, or Faction)
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdStandingsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CharacterStandingType {
    /// Standing type is with an NPC agent
    #[serde(rename = "agent")]
    Agent,
    /// Standing type is with an NPC corporation
    #[serde(rename = "npc_corp")]
    NpcCorp,
    /// Standing type is with an NPC faction
    #[serde(rename = "faction")]
    Faction,
}
