//! # EVE ESI Clone Models
//!
//! Provides clone-related structs for EVE Online
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>
//!
//! ## Models
//! | Model               | Description                     |
//! | ------------------- | ------------------------------- |
//! | [`AllianceContact`] | A contact entry for an alliance |

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
    pub is_blocked: bool,
    /// If true, character is on buddy list
    ///
    /// Note: There used to be a watchlist a long time ago to notify when someone is online but it has since been
    /// changed to buddy list which requires the characters to mutual set each other as buddy AKA watched to see online notification.
    pub is_watched: bool,
    /// List of unique IDs applied to the contact entry
    pub label_ids: Vec<i64>,
    /// Standings towards the contact
    pub standing: f64,
}
