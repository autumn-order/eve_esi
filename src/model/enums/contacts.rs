//! # EVE ESI Clone Enums
//!
//! Provides clone-related enums for EVE Online
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>
//!
//! ## Enums
//! | Enum | Description
//! - [`CharacterCloneLocationType`]: The location type of a character's clone

use serde::{Deserialize, Serialize};

/// Represents the type of contact
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/AlliancesAllianceIdContactsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ContactType {
    /// Contact type is a character
    #[serde(rename = "character")]
    Character,
    /// Contact type is a corporation
    #[serde(rename = "corporation")]
    Corporation,
    /// Contact type is an alliance
    #[serde(rename = "alliance")]
    Alliance,
    /// Contact type is an NPC faction
    #[serde(rename = "faction")]
    Faction,
}
