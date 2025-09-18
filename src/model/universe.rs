//! # EVE ESI Universe Models
//!
//! This module defines models shared between entities in EVE Online

use serde::{Deserialize, Serialize};

use crate::model::enums::universe::StandingType;

/// A character or corporation's standings with either an agent, NPC corp, or faction
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdStandingsGet>
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStandingsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Standing {
    /// The ID of the entity the standing is with
    pub from_id: i64,
    /// The type of entity the standing entry is with (Agent, NpcCorp, or Faction)
    pub from_type: StandingType,
    /// The character's standing with the listed entity
    pub standing: f64,
}
