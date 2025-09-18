//! Data structures and types for representing corporations in EVE Online.
//!
//! This module defines the `Corporation` struct, which models the core properties of a corporation in EVE Online.
//!
//! See [ESI API documentation](https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdGet)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::{asset::LocationFlag, corporation::CorporationSecureContainerAction};

/// Represents a corporation in EVE Online.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Corporation {
    /// The ID of the alliance which the corporation is a member of, if applicable.
    pub alliance_id: Option<i64>,
    /// The ID of the corporation's CEO.
    pub ceo_id: i64,
    /// The ID of the character who created the corporation.
    pub creator_id: i64,
    /// The date and time when the corporation was founded.
    pub date_founded: Option<DateTime<Utc>>,
    /// The description of the corporation.
    pub description: Option<String>,
    /// The ID of the faction which the corporation is a member of, if applicable.
    pub faction_id: Option<i64>,
    /// The ID of the corporation's home station.
    pub home_station_id: Option<i64>,
    /// The number of members in the corporation.
    pub member_count: i64,
    /// The name of the corporation.
    pub name: String,
    /// The number of shares that exist for the corporation.
    pub shares: Option<i64>,
    /// The tax rate of the corporation.
    pub tax_rate: f64,
    /// The ticker of the corporation.
    pub ticker: String,
    /// The URL of the corporation's website.
    pub url: Option<String>,
    /// Whether or not the corporation is eligible for war.
    pub war_eligible: Option<bool>,
}

/// Entry for a corporation's alliance history
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdAlliancehistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationAllianceHistory {
    /// The ID of the alliance
    pub alliance_id: i64,
    /// Bool indicating whether or not the alliance has been closed
    #[serde(default)]
    pub is_deleted: bool,
    /// An incrementing ID representing the order of the alliance in the history
    pub record_id: i64,
    /// The date of when the corporation joined the alliance
    pub start_date: DateTime<Utc>,
}

/// Log entry for an audit log secure container owned by a corporation
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdAlliancehistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationSecureContainerLog {
    /// The action taken on the container
    pub action: CorporationSecureContainerAction,
    /// ID of the character who performed the action
    pub character_id: i64,
    /// ID of the container
    pub container_id: i64,
    /// Type ID of the container
    pub container_type_id: i64,
    /// Indicates the type of location for the container
    pub location_flag: LocationFlag,
    /// The ID of the container's location
    pub location_id: i64,
    /// Timestamp when this log was created
    pub logged_at: DateTime<Utc>,
    /// ???
    pub new_config_bitmask: i64,
    /// ???
    pub old_config_bitmask: i64,
    /// Quantity of item being acted upon
    pub quantity: i64,
    /// Type ID of the item being acted upon
    pub type_id: i64,
}

/// Log entry for an audit log secure container owned by a corporation
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdDivisionsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationDivisionEntry {
    /// The number of the division (1-7), for wallet divisions: 1 = master wallet
    pub division: i64,
    /// Name of the division
    pub name: String,
}

/// Log entry for an audit log secure container owned by a corporation
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdDivisionsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationDivisions {
    /// List of hangar division entries
    pub hangar: Vec<CorporationDivisionEntry>,
    /// List of wallet division entries
    pub wallet: Vec<CorporationDivisionEntry>,
}
