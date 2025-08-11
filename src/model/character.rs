use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a character in EVE Online.
///
/// This struct contains the basic information about a character as returned by the EVE ESI API.
///
/// # Fields
/// The struct contains identifying information about the character, including:
/// - Alliance and corporation affiliations
/// - Birthday (creation date) of the character
/// - Demographic information such as bloodline, race, and gender
/// - Optional descriptive elements such as character description and title
/// - Security status information if available
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Character {
    /// The ID of the alliance the character belongs to, if applicable.
    pub alliance_id: Option<i32>,
    /// The date and time when the character was created.
    pub birthday: DateTime<Utc>,
    /// The ID representing the character's bloodline, which determines cosmetic appearance.
    pub bloodline_id: i32,
    /// The ID of the corporation the character is a member of.
    pub corporation_id: i32,
    /// An optional self-written description of the character.
    pub description: Option<String>,
    /// The ID of the faction the character is associated with, if applicable.
    pub faction_id: Option<i32>,
    /// The gender of the character.
    pub gender: String,
    /// The name of the character.
    pub name: String,
    /// The ID representing the character's race (Amarr, Caldari, Gallente, or Minmatar).
    pub race_id: i32,
    /// The security status of the character, ranging from -10 to +10.
    pub security_status: Option<f32>,
    /// An optional title displayed for the character.
    pub title: Option<String>,
}

/// Represents the organizational affiliations of a character in EVE Online.
///
/// This struct contains information about a character's membership in corporations,
/// alliances, and factions within the EVE universe, as returned by the EVE ESI API.
///
/// # Fields
/// The struct identifies all organizational entities the character belongs to:
/// - The character's unique ID
/// - The corporation the character is a member of
/// - The alliance the corporation belongs to, if applicable
/// - The faction the character is associated with, if applicable
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterAffiliation {
    /// The ID of the alliance the character belongs to, if applicable.
    pub alliance_id: Option<i32>,
    /// The unique identifier for this character.
    pub character_id: i32,
    /// The ID of the corporation the character is a member of.
    pub corporation_id: i32,
    /// The ID of the faction the character is associated with, if applicable.
    pub faction_id: Option<i32>,
}
