//! Data structures and types for representing characters in EVE Online.
//!
//! This module defines the `Character` & `CharacterAffiliation` structs,
//! which model the core properties of a character & character affiliation in EVE Online.
//!
//! See [ESI API documentation](https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdGet)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub faction_id: Option<i32>,
}
