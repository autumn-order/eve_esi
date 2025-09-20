//! # EVE ESI Character Models
//!
//! This module defines the `Character` & `CharacterAffiliation` structs,
//! which model the core properties of a character & character affiliation in EVE Online.
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Models
//! - [`Character`]: Represents a character in EVE Online
//! - [`CharacterAffiliation`]: Represents the affiliations of a character in EVE Online
//! - [`CharacterResearchAgent`]: Information regarding a character's research agent
//! - [`CharacterCorporationHistory`]: Represents a character's corporation history
//! - [`CharacterJumpFatigue`]: Represents a character's jump fatigue status
//! - [`CharacterMedalGraphics`]: Represents the graphics configuration for a character's medal
//! - [`CharacterMedal`]: Represents an entry for a character's medals
//! - [`CharacterNotification`]: Represents a character notification entry
//! - [`CharacterNewContactNotification`]: Notification when character has been added to someone's contact list
//! - [`CharacterPortraits`]: A character's portrait URLs with various dimensions
//! - [`CharacterCorporationRole`]: A character's portrait URLs with various dimensions
//! - [`CharacterCorporationTitle`]: An entry for a character's corporation titles

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::{
    character::CharacterMedalStatus,
    corporation::CorporationRole,
    notification::{NotificationSenderType, NotificationType},
};

/// Represents a character in EVE Online
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

/// Represents the affiliations of a character in EVE Online
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
    pub faction_id: Option<i64>,
}

/// Information regarding a character's research agent
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAgentsResearchGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterResearchAgent {
    /// ID of the research agent
    pub agent_id: i64,
    /// Research points accumulated per day
    pub points_per_day: f64,
    /// Remaining research points available to spend
    pub remainder_points: f64,
    /// Specialty skill of the research agent
    pub skill_type_id: i64,
    /// Date and time the research agent was started
    pub started_at: DateTime<Utc>,
}

/// Represents a character's corporation history
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdCorporationhistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterCorporationHistory {
    /// The ID of the corporation
    pub corporation_id: i64,
    /// Bool indicating whether or not corporation has been deleted
    #[serde(default)]
    pub is_deleted: bool,
    /// An incrementing ID representing the order of the corporation in the history
    pub record_id: i64,
    /// The date of when the character joined the corporation
    pub start_date: DateTime<Utc>,
}

/// Represents a character's jump fatigue status
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdFatigueGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterJumpFatigue {
    /// Character's jump fatigue expiry
    pub jump_fatigue_expire_date: DateTime<Utc>,
    /// Character's last jump activation
    pub last_jump_date: DateTime<Utc>,
    /// Character's last jump update
    pub last_update_date: DateTime<Utc>,
}

/// Represents the graphics configuration for a character's medal
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdMedalsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterMedalGraphics {
    /// Color of the medal graphic
    pub color: i64,
    /// The name of the graphic
    pub graphic: String,
    /// The layer of the graphic on the medal
    pub layer: i64,
    /// The part of the medal the graphic is on
    pub part: i64,
}

/// Represents an entry for a character's medals
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdMedalsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterMedal {
    /// The corporation which had issued the medal
    pub corporation_id: i64,
    /// The date which the medal was issued
    pub date: DateTime<Utc>,
    /// Description for the medal
    pub description: String,
    /// The graphic configuration of the medal
    pub graphics: Vec<CharacterMedalGraphics>,
    /// The character who had issued the medal
    pub issuer_id: i64,
    /// The unique ID of the medal
    pub medal_id: i64,
    /// The reason the medal was issued
    pub reason: String,
    /// The visibility status of the medal
    pub status: CharacterMedalStatus,
    /// The name of the medal
    pub title: String,
}

/// Represents a character notification entry
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdNotificationsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterNotification {
    /// Whether or not the notification has been read
    #[serde(default)]
    pub is_read: bool,
    /// The unique ID of the notification
    pub notification_id: i64,
    /// ID of the sender if applicable (character, corporation, alliance, faction, other)
    pub sender_id: i64,
    /// The type of sender if applicable (character, corporation, alliance, faction, other)
    pub sender_type: Option<NotificationSenderType>,
    /// The text content of the notification
    pub text: Option<String>,
    /// The timestamp the notification was sent
    pub timestamp: DateTime<Utc>,
    /// The type of notification
    pub r#type: NotificationType,
}

/// Notification when character has been added to someone's contact list
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdNotificationsContactsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterNewContactNotification {
    /// The message content of the notification
    pub message: String,
    /// The unique ID of the notification
    pub notification_id: i64,
    /// The timestamp when the notification was sent
    pub send_date: DateTime<Utc>,
    /// The character ID of the notification sender
    pub sender_character_id: i64,
    /// Number representing contact standing level with the sender
    pub standing_level: f64,
}

/// A character's portrait URLs with various dimensions
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdPortraitGet>
///
/// Note: ESI documentation shows all fields of this struct as optional, this may be misdocumented,
/// if any deserialization errors regarding this struct occur please open an issue on the eve_esi crate
/// GitHub repository. Fields will be set as not optional for the time being.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterPortraits {
    /// Character's portrait URL in 64x64px
    pub px64x64: String,
    /// Character's portrait URL in 128x128px
    pub px128x128: String,
    /// Character's portrait URL in 256x256px
    pub px256x256: String,
    /// Character's portrait URL in 512x512px
    pub px512x512: String,
}

/// A character's portrait URLs with various dimensions
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdRolesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterCorporationRole {
    /// Roles applicable across the entire corp
    pub roles: Vec<CorporationRole>,
    /// Roles only applicable to a corporation base
    pub roles_at_base: Vec<CorporationRole>,
    /// Roles applicable to only the corporation HQ
    pub roles_at_hq: Vec<CorporationRole>,
    /// Roles applicable at all other locations
    pub roles_at_other: Vec<CorporationRole>,
}

/// An entry for a character's corporation titles
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdTitlesGet>
///
/// Note: ESI documentation shows all fields of this struct as optional, this may be misdocumented,
/// if any deserialization errors regarding this struct occur please open an issue on the eve_esi crate
/// GitHub repository. Fields will be set as not optional for the time being.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterCorporationTitle {
    /// The title content
    pub name: String,
    /// The unique ID of the title
    pub title_id: i64,
}
