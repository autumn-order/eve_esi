//! # EVE ESI Clone Models
//!
//! Provides clone-related structs for EVE Online
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>
//!
//! ## Models
//! | Model                  | Description                                                                 |
//! | ---------------------- | --------------------------------------------------------------------------- |
//! | [`AllianceContact`]    | A contact entry for an alliance                                             |
//! | [`ContactLabel`]       | A contact label entry shared across alliances, corporations, and characters |
//! | [`CharacterContact`]   | A contact entry for character                                               |
//! | [`CorporationContact`] | A contact entry for a corporation                                           |

use serde::{Deserialize, Serialize};

use crate::model::enums::contacts::ContactType;

/// A contact entry for an alliance
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/AlliancesAllianceIdContactsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AllianceContact {
    /// Unique ID of the contact
    pub contact_id: i64,
    /// Indicates whether contact ID is a `character`, `corporation`, `alliance`, or `faction`
    pub contact_type: ContactType,
    /// List of unique IDs applied to the contact entry
    #[serde(default)]
    pub label_ids: Vec<i64>,
    /// Standings towards the contact
    pub standing: f64,
}

/// A contact label entry shared across alliances, corporations, and characters
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/AlliancesAllianceIdContactsLabelsGet>
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdContactsLabelsGet>
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdContactsLabelsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContactLabel {
    /// Unique ID of the contact label
    pub label_id: i64,
    /// Name of the label
    pub label_name: String,
}

/// A contact entry for character
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdContactsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterContact {
    /// Unique ID of the contact
    pub contact_id: i64,
    /// Indicates whether contact ID is a `character`, `corporation`, `alliance`, or `faction`
    pub contact_type: ContactType,
    /// If true, character is blocked
    #[serde(default)]
    pub is_blocked: bool,
    /// If true, character is on buddy list
    ///
    /// Note: There used to be a watchlist a long time ago to notify when someone is online but it has since been
    /// changed to buddy list which requires the characters to mutual set each other as buddy AKA watched to see online notification.
    #[serde(default)]
    pub is_watched: bool,
    /// List of unique IDs applied to the contact entry
    #[serde(default)]
    pub label_ids: Vec<i64>,
    /// Standings towards the contact
    pub standing: f64,
}

/// A contact entry for a corporation
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdContactsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationContact {
    /// Unique ID of the contact
    pub contact_id: i64,
    /// Indicates whether contact ID is a `character`, `corporation`, `alliance`, or `faction`
    pub contact_type: ContactType,
    // Unknown if this applicable to corporations with the deprecation of the original watchlist, ESI defines this as part of the
    // get corporation contact response body so it has been included just in case.
    /// If true, character is on buddy list
    ///
    /// Note: There used to be a watchlist a long time ago to notify when someone is online but it has since been
    /// changed to buddy list which requires the characters to mutual set each other as buddy AKA watched to see online notification.
    #[serde(default)]
    pub is_watched: bool,
    /// List of unique IDs applied to the contact entry
    #[serde(default)]
    pub label_ids: Vec<i64>,
    /// Standings towards the contact
    pub standing: f64,
}
