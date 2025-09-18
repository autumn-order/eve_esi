//! Data structures and types for representing corporations in EVE Online.
//!
//! This module defines the `Corporation` struct, which models the core properties of a corporation in EVE Online.
//!
//! See [ESI API documentation](https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdGet)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::{
    asset::LocationFlag,
    character::CharacterMedalStatus,
    corporation::{
        CorporationRole, CorporationRoleType, CorporationSecureContainerAction, ShareholderType,
    },
};

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

/// Entry for corporation industry facilities
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdDivisionsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationFacilities {
    /// ID of the facility
    pub facility_id: i64,
    /// ID of the system where the facility is located
    pub system_id: i64,
    /// Type ID of the facility
    pub type_id: i64,
}

/// Icon URLs for a corporation
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdIconsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationIcon {
    px128x128: String,
    px256x256: String,
    px64x64: String,
}

/// An entry for a corporation medal
///
/// This model differs from [`CorporationIssuedMedal`] in that it describes the medal itself
/// while [`CorporationIssuedMedal`] represents who issued the medal and who the medal was issued to.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdMedalsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMedal {
    /// The timestamp of when the medal was created
    created_at: DateTime<Utc>,
    /// ID of the character who created the medal
    creator_id: i64,
    /// The description of the medal
    description: String,
    /// The unique ID of the medal
    medal_id: i64,
    /// The name of the medal
    title: String,
}

/// An entry for an issued corporation medal
///
/// This model differs from [`CorporationMedal`] in that it represents who issued the medal and
/// who the medal was issued to while [`CorporationMedal`] describes the medals themselves.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdMedalsIssuedGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationIssuedMedal {
    /// ID of the character who was granted the medal
    character_id: i64,
    /// The timestamp of when the medal was created
    issued_at: DateTime<Utc>,
    /// ID of the character who issued the medal
    issuer_id: i64,
    /// The unique ID of the medal
    medal_id: i64,
    /// The reason the medal was issued
    reason: String,
    /// Whether the character's medal visibility is public or private
    status: CharacterMedalStatus,
}

/// An entry for a corporation member's titles
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdMembersTitlesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberTitles {
    /// ID of the character who the titles belong to
    character_id: i64,
    /// List of title IDs belonging to the character
    titles: Vec<i64>,
}

/// An entry for a corporation member's tracking information
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdMembertrackingGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberTracking {
    /// Corporation base ID which the character belongs to
    base_id: i64,
    /// ID of the character who the tracking information belongs to
    character_id: i64,
    /// ID of the character's current location
    location_id: i64,
    /// Last logoff date of the character
    logoff_date: DateTime<Utc>,
    /// Last logon date of the character
    logon_date: DateTime<Utc>,
    /// Type ID of the ship the character is currently in
    ship_type_id: i64,
    /// The date when the member joined their current corporation
    start_date: DateTime<Utc>,
}

/// An entry for a corporation member's assigned roles
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdRolesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberRoles {
    /// ID of character who the roles belong to
    character_id: i64,
    /// Roles character is capable of granting corporation-wide
    grantable_roles: Vec<CorporationRole>,
    /// Roles character is capable of granting at their assigned base
    grantable_roles_at_base: Vec<CorporationRole>,
    /// Roles character is capable of granting at corporation HQ
    grantable_roles_at_hq: Vec<CorporationRole>,
    /// Roles character is capable of granting at other locations
    grantable_roles_at_other: Vec<CorporationRole>,
    /// Roles character holds corporation-wide
    roles: Vec<CorporationRole>,
    /// Roles character holds at their assigned base
    roles_at_base: Vec<CorporationRole>,
    /// Roles character holds at corporation HQ
    roles_at_hq: Vec<CorporationRole>,
    /// Roles character holds at other locations
    roles_at_other: Vec<CorporationRole>,
}

/// An entry for a corporation member's role history
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdRolesHistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberRolesHistory {
    /// Timestamp of the role change log entry
    changed_at: DateTime<Utc>,
    /// ID of character who the roles belong to
    character_id: i64,
    /// ID of character who modified the roles
    issuer_id: i64,
    /// New roles now held by the character
    new_roles: Vec<CorporationRole>,
    /// Old roles previously held by character
    old_roles: Vec<CorporationRole>,
    /// The location of the roles
    role_type: CorporationRoleType,
}

/// An entry for a corporation shareholder
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdShareholdersGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationShareholder {
    /// Amount of shares held by the shareholder
    share_count: i64,
    /// ID of the character or corporation who holds the shares
    shareholder_id: i64,
    /// Represents whether the shares are held by a character or corporation
    shareholder_type: ShareholderType,
}
