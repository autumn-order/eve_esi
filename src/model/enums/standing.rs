//! # EVE ESI Standing Enums
//!
//! Provides standing enum shared between characters & corporations
//!
//! ## Enums
//! - [`StandingType`]: The type of character or corporation standing entry (Agent, NpcCorp, or Faction)

use serde::{Deserialize, Serialize};

/// The type of character or corporation standing entry (Agent, NpcCorp, or Faction)
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdStandingsGet>
///
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StandingType {
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
