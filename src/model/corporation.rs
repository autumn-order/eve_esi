//! # EVE ESI Corporation Models
//!
//! This module defines the `Corporation` struct, which models the core properties of a corporation in EVE Online.
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Models
//! - [`Corporation`]: Represents a corporation in EVE Online
//! - [`CorporationAllianceHistory`]: Entry for a corporation's alliance history
//! - [`CorporationSecureContainerLog`]: Log entry for an audit log secure container owned by a corporation
//! - [`CorporationDivisionEntry`]: An entry for a corporation's hangar or wallet division
//! - [`CorporationDivisions`]: Lists of a corporation wallet and hangar divisions
//! - [`CorporationFacilities`]: Entry for corporation industry facilities
//! - [`CorporationIcon`]: Icon URLs for a corporation
//! - [`CorporationMedal`]: An entry for a corporation medal
//! - [`CorporationIssuedMedal`]: An entry for an issued corporation medal
//! - [`CorporationMemberTitles`]: An entry for a corporation member's titles
//! - [`CorporationMemberTracking`]: An entry for a corporation member's tracking information
//! - [`CorporationMemberRoles`]: An entry for a corporation member's assigned roles
//! - [`CorporationMemberRolesHistory`]: An entry for a corporation member's role history
//! - [`CorporationShareholder`]: An entry for a corporation shareholder
//! - [`CorporationStarbase`]: Information regarding a starbase (POS) owned by a corporation
//! - [`CorporationStarbaseFuel`]: Entry on the fuel types stored within a corporation starbase (POS)
//! - [`CorporationStarbaseDetails`]: Information regarding a starbase's (POS) details owned by a corporation
//! - [`CorporationStructureService`]: An entry for a corporation's Upwell structure services
//! - [`CorporationStructure`]: Details regarding a corporation's Upwell structure
//! - [`CorporationTitle`]: An entry for a corporation's titles and its respective roles

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::{
    asset::LocationFlag,
    character::CharacterMedalStatus,
    corporation::{
        CorporationRole, CorporationRoleType, CorporationSecureContainerAction,
        CorporationStarbasePermission, CorporationStarbaseState, CorporationStructureServiceState,
        CorporationStructureState, ShareholderType,
    },
};

/// Represents a corporation in EVE Online
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
    /// The ID of the alliance if applicable
    pub alliance_id: Option<i64>,
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

/// An entry for a corporation's hangar or wallet division
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdDivisionsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationDivisionEntry {
    /// The number of the division (1-7), for wallet divisions: 1 = master wallet
    pub division: i64,
    /// Name of the division, None if divison has not been named
    pub name: Option<String>,
}

/// Lists of a corporation wallet and hangar divisions
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
///
/// Note: ESI documentation shows all fields of this struct as optional, this may be misdocumented,
/// if any deserialization errors regarding this struct occur please open an issue on the eve_esi crate
/// GitHub repository. Fields will be set as not optional for the time being.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationIcon {
    /// URL of the 128x128 px variant of the corporation's logo
    pub px128x128: String,
    /// URL of the 256x256 px variant of the corporation's logo
    pub px256x256: String,
    /// URL of the 64x64 px variant of the corporation's logo
    pub px64x64: String,
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
    pub created_at: DateTime<Utc>,
    /// ID of the character who created the medal
    pub creator_id: i64,
    /// The description of the medal
    pub description: String,
    /// The unique ID of the medal
    pub medal_id: i64,
    /// The name of the medal
    pub title: String,
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
    pub character_id: i64,
    /// The timestamp of when the medal was created
    pub issued_at: DateTime<Utc>,
    /// ID of the character who issued the medal
    pub issuer_id: i64,
    /// The unique ID of the medal
    pub medal_id: i64,
    /// The reason the medal was issued
    pub reason: String,
    /// Whether the character's medal visibility is public or private
    pub status: CharacterMedalStatus,
}

/// An entry for a corporation member's titles
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdMembersTitlesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberTitles {
    /// ID of the character who the titles belong to
    pub character_id: i64,
    /// List of title IDs belonging to the character
    pub titles: Vec<i64>,
}

/// An entry for a corporation member's tracking information
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdMembertrackingGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberTracking {
    /// Corporation base ID which the character belongs to
    pub base_id: Option<i64>,
    /// ID of the character who the tracking information belongs to
    pub character_id: i64,
    /// ID of the character's current location
    pub location_id: Option<i64>,
    /// Last logoff date of the character
    pub logoff_date: Option<DateTime<Utc>>,
    /// Last logon date of the character
    pub logon_date: Option<DateTime<Utc>>,
    /// Type ID of the ship the character is currently in
    pub ship_type_id: Option<i64>,
    /// The date when the member joined their current corporation
    pub start_date: Option<DateTime<Utc>>,
}

/// An entry for a corporation member's assigned roles
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdRolesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberRoles {
    /// ID of character who the roles belong to
    pub character_id: i64,
    /// Roles character is capable of granting corporation-wide
    pub grantable_roles: Vec<CorporationRole>,
    /// Roles character is capable of granting at their assigned base
    pub grantable_roles_at_base: Vec<CorporationRole>,
    /// Roles character is capable of granting at corporation HQ
    pub grantable_roles_at_hq: Vec<CorporationRole>,
    /// Roles character is capable of granting at other locations
    pub grantable_roles_at_other: Vec<CorporationRole>,
    /// Roles character holds corporation-wide
    pub roles: Vec<CorporationRole>,
    /// Roles character holds at their assigned base
    pub roles_at_base: Vec<CorporationRole>,
    /// Roles character holds at corporation HQ
    pub roles_at_hq: Vec<CorporationRole>,
    /// Roles character holds at other locations
    pub roles_at_other: Vec<CorporationRole>,
}

/// An entry for a corporation member's role history
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdRolesHistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMemberRolesHistory {
    /// Timestamp of the role change log entry
    pub changed_at: DateTime<Utc>,
    /// ID of character who the roles belong to
    pub character_id: i64,
    /// ID of character who modified the roles
    pub issuer_id: i64,
    /// New roles now held by the character
    pub new_roles: Vec<CorporationRole>,
    /// Old roles previously held by character
    pub old_roles: Vec<CorporationRole>,
    /// The location of the roles
    pub role_type: CorporationRoleType,
}

/// An entry for a corporation shareholder
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdShareholdersGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationShareholder {
    /// Amount of shares held by the shareholder
    pub share_count: i64,
    /// ID of the character or corporation who holds the shares
    pub shareholder_id: i64,
    /// Represents whether the shares are held by a character or corporation
    pub shareholder_type: ShareholderType,
}

/// Information regarding a starbase (POS) owned by a corporation
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStarbasesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationStarbase {
    /// Optional ID of the moon which the starbase (POS) is anchored on,
    /// will be None if POS is unanchored
    pub moon_id: Option<i64>,
    /// When the POS is onlined,
    /// only applicable if [`CorporationStarbase::state`] is [`CorporationStarbaseState::Onlining`]
    pub onlined_since: Option<DateTime<Utc>>,
    /// When the POS is reinforced until,
    /// only applicable if [`CorporationStarbase::state`] is [`CorporationStarbaseState::Reinforced`]
    pub reinforced_until: Option<DateTime<Utc>>,
    /// Unique ID of the starbase (POS)
    pub starbase_id: i64,
    /// Enum representing the current state of the starbase (POS), None if unanchored.
    pub state: Option<CorporationStarbaseState>,
    /// Optional ID of the solar system the starbase (POS) is located,
    /// will be None if POS is unanchored
    pub system_id: Option<i64>,
    /// The ID of the type of starbase (POS)
    pub type_id: i64,
    /// The time when the POS started unanchoring,
    /// only applicable if [`CorporationStarbase::state`] is [`CorporationStarbaseState::Unanchoring`]
    pub unanchor_at: Option<DateTime<Utc>>,
}

/// Entry on the fuel types stored within a corporation starbase (POS)
///
/// # ESI Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStarbasesStarbaseIdGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationStarbaseFuel {
    /// The quantity of fuel stored in the starbase (POS)
    pub quantity: i64,
    /// The type ID of the fuel stored within the starbase (POS)
    pub type_id: i64,
}

/// Information regarding a starbase's (POS) details owned by a corporation
///
/// # ESI Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStarbasesStarbaseIdGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationStarbaseDetails {
    /// Allow alliance members to access POS
    pub allow_alliance_members: bool,
    /// Allow corporation members to access POS
    pub allow_corporation_members: bool,
    /// Enum indicating who has permission to anchor structures for POS
    pub anchor: CorporationStarbasePermission,
    /// Boolean indicating whether or not for POS to automatically attack pilots at war with owning corporation
    pub attack_if_at_war: bool,
    /// Boolean indicating whether or not for POS to automatically attack pilots with low sec status
    pub attack_if_other_security_status_dropping: bool,
    /// POS will attack pilots automatically if their security status is below threshold
    pub attack_security_status_threshold: Option<f64>,
    /// POS will attack pilots automatically if their standings are below threshold
    pub attack_standing_threshold: Option<f64>,
    /// Enum indicating who has permission to take from POS fuel bay
    pub fuel_bay_take: CorporationStarbasePermission,
    /// Enum indicating who has permission to view POS fuel bay
    pub fuel_bay_view: CorporationStarbasePermission,
    /// List of entries containing type_id of fuel and quantity stored within the POS
    pub fuels: Vec<CorporationStarbaseFuel>,
    /// Enum indicating who has permission to offline POS and its structures
    pub offline: CorporationStarbasePermission,
    /// Enum indicating who has permission to online POS and its structures
    pub online: CorporationStarbasePermission,
    /// Enum indicating who has permission to unachor POS and its structures
    pub unanchor: CorporationStarbasePermission,
    /// Boolean which if true will use alliance standings, otherwise using corporation's
    pub use_alliance_standings: bool,
}

/// An entry for a corporation's Upwell structure services
///
/// # ESI Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStructuresGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationStructureService {
    /// The name of the structure service
    pub name: String,
    /// Enum representing the state of the structure service
    pub state: CorporationStructureServiceState,
}

/// Details regarding a corporation's Upwell structure
///
/// # ESI Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStructuresGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationStructure {
    /// ID of the corporation that owns the structures
    pub corporation_id: i64,
    /// Timestamp when the structure will run out of fuel
    pub fuel_expires: Option<DateTime<Utc>>,
    /// Name of the structure
    pub name: Option<String>,
    /// The timestamp of when the structure's newly set reinforcement period will take effect
    pub next_reinforce_apply: Option<DateTime<Utc>>,
    /// The reinforcement hour once the new reinforcement period takes effect
    pub next_reinforce_hour: Option<i64>,
    /// The ID of the ACL profile for the structure
    pub profile_id: i64,
    /// The current reinforce hour of the structure that determines when the structure will exit
    /// reinforcement and become vulnerable to attack for an armor or hull timer.
    pub reinforce_hour: Option<i64>,
    /// A list of entries for structure services
    pub services: Vec<CorporationStructureService>,
    /// An enum representing the current state of the structure
    pub state: CorporationStructureState,
    /// The timestamp when the structure will move to its next state
    pub state_timer_end: Option<DateTime<Utc>>,
    /// The timestamp when the structure entered its current state
    pub state_timer_start: Option<DateTime<Utc>>,
    /// The unique ID of the structure
    pub structure_id: i64,
    /// The ID of the system where the structure is located
    pub system_id: i64,
    /// The type ID of the structure
    pub type_id: i64,
    /// The timestamp when the structure will unanchor
    pub unanchors_at: Option<DateTime<Utc>>,
}

/// An entry for a corporation's titles and its respective roles
///
/// # ESI Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdTitlesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationTitle {
    /// Roles title is capable of granting corporation-wide
    pub grantable_roles: Vec<CorporationRole>,
    /// Roles title is capable of granting at their assigned base
    pub grantable_roles_at_base: Vec<CorporationRole>,
    /// Roles title is capable of granting at corporation HQ
    pub grantable_roles_at_hq: Vec<CorporationRole>,
    /// Roles title is capable of granting at other locations
    pub grantable_roles_at_other: Vec<CorporationRole>,
    /// Name of the title
    pub name: String,
    /// Roles title has corporation-wide
    pub roles: Vec<CorporationRole>,
    /// Roles title has at their assigned base
    pub roles_at_base: Vec<CorporationRole>,
    /// Roles title has at corporation HQ
    pub roles_at_hq: Vec<CorporationRole>,
    /// Roles title has at other locations
    pub roles_at_other: Vec<CorporationRole>,
    /// ID of the title
    pub title_id: i64,
}
