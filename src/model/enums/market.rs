//! # EVE ESI Market Enums
//!
//! Provides enums related to markets in EVE Online
//!
//! ## Enums
//! - [`CharacterMedalStatus`]: Represents the visbility status of a character's medal

use serde::{Deserialize, Serialize};

/// Indicates the the range of a market order
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdOrdersGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MarketOrderRange {
    #[serde(rename = "1")]
    OneJump,
    #[serde(rename = "2")]
    TwoJumps,
    #[serde(rename = "3")]
    ThreeJumps,
    #[serde(rename = "4")]
    FourJumps,
    #[serde(rename = "5")]
    FiveJumps,
    #[serde(rename = "10")]
    TenJumps,
    #[serde(rename = "20")]
    TwentyJumps,
    #[serde(rename = "30")]
    ThirtyJumps,
    #[serde(rename = "40")]
    FourtyJumps,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "solarsystem")]
    SolarSystem,
    #[serde(rename = "station")]
    Station,
}
