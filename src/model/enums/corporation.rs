//! # EVE ESI Corporation Enums
//!
//! Provides enums related to corporations in EVE Online

use serde::{Deserialize, Serialize};

/// Indicates the type & location of the corporation role
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdRolesHistoryGet>
/// - <https://support.eveonline.com/hc/en-us/articles/203217712-Roles-Listing> (See location identifier setion)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CorporationRoleType {
    /// Roles character is capable of granting corporation-wide
    #[serde(rename = "grantable_roles")]
    GrantableRoles,
    /// Roles character is capable of granting at their assigned base
    #[serde(rename = "grantable_roles_at_base")]
    GrantableRolesAtBase,
    /// Roles character is capable of granting at corporation HQ
    #[serde(rename = "grantable_roles_at_hq")]
    GrantableRolesAtHq,
    /// Roles character is capable of granting at other locations
    #[serde(rename = "grantable_roles_at_other")]
    GrantableRolesAtOther,
    /// Roles character holds corporation-wide
    #[serde(rename = "roles")]
    Roles,
    /// Roles character holds at their assigned base
    #[serde(rename = "roles_at_base")]
    RolesAtBase,
    /// Roles character holds at corporation HQ
    #[serde(rename = "roles_at_hq")]
    RolseAtHq,
    /// Roles character holds at other locations
    #[serde(rename = "roles_at_other")]
    RolesAtOther,
}

/// Indicates the type of corporation role
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdRolesGet>
/// - <https://support.eveonline.com/hc/en-us/articles/203217712-Roles-Listing>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CorporationRole {
    /// Access to take funds from master corporation wallet
    #[serde(rename = "Account_Take_1")]
    AccountTake1,
    /// Access to take funds from corporaiton wallet division #2
    #[serde(rename = "Account_Take_2")]
    AccountTake2,
    /// Access to take funds from corporaiton wallet division #3
    #[serde(rename = "Account_Take_3")]
    AccountTake3,
    /// Access to take funds from corporaiton wallet division #4
    #[serde(rename = "Account_Take_4")]
    AccountTake4,
    /// Access to take funds from corporaiton wallet division #5
    #[serde(rename = "Account_Take_5")]
    AccountTake5,
    /// Access to take funds from corporaiton wallet division #6
    #[serde(rename = "Account_Take_6")]
    AccountTake6,
    /// Access to take funds from corporaiton wallet division #7
    #[serde(rename = "Account_Take_7")]
    AccountTake7,
    /// View-only access to wallets & transactions
    Accountant,
    /// View-only access to the auditing tab of the corporation management screen to see role
    /// assignments & removals
    Auditor,
    /// Ability to create & apply SKINR to Upwell structures
    #[serde(rename = "Brand_Manager")]
    BrandManager,
    /// Ability to send corporation/alliance evemails as well as modify corporation chat MOTDz
    #[serde(rename = "Communications_Officer")]
    CommunicationsOfficer,
    /// Ability to deploy & configure deployables on behalf of the corporation, does not
    /// apply to Upwell Structures which instead require [`CorporationRole::StationManager`]
    #[serde(rename = "Config_Equipment")]
    ConfigEquipment,
    /// Ability to modify POS tower equipment
    #[serde(rename = "Config_Starbase_Equipment")]
    ConfigStarbaseEquipment,
    /// Ability to remove items from containers within corporation hangar divison #1
    #[serde(rename = "Container_Take_1")]
    ContainerTake1,
    /// Ability to remove items from containers within corporation hangar divison #2
    #[serde(rename = "Container_Take_2")]
    ContainerTake2,
    /// Ability to remove items from containers within corporation hangar divison #3
    #[serde(rename = "Container_Take_3")]
    ContainerTake3,
    /// Ability to remove items from containers within corporation hangar divison #4
    #[serde(rename = "Container_Take_4")]
    ContainerTake4,
    /// Ability to remove items from containers within corporation hangar divison #5
    #[serde(rename = "Container_Take_5")]
    ContainerTake5,
    /// Ability to remove items from containers within corporation hangar divison #6
    #[serde(rename = "Container_Take_6")]
    ContainerTake6,
    /// Ability to remove items from containers within corporation hangar divison #7
    #[serde(rename = "Container_Take_7")]
    ContainerTake7,
    /// Access to manage corporation contracts and contracts assigned to the corporation
    #[serde(rename = "Contract_Manager")]
    ContractManager,
    /// Ability to remove items from containers within the corporation deliveries hangar
    #[serde(rename = "Deliveries_Container_Take")]
    DeliveriesContainerTake,
    /// View-only access to the corporation deliveries hangar
    #[serde(rename = "Deliveries_Query")]
    DeliveriesQuery,
    /// Ability to remove items from the corporation deliveries hangar
    #[serde(rename = "Deliveries_Take")]
    DeliveriesTake,
    /// Access to manage corporation standings
    Diplomat,
    /// Access to all corporation permissions aside from those exclusive to CEO
    Director,
    #[serde(rename = "Factory_Manager")]
    /// Access to manage corporation industry jobs, even those created by other members
    FactoryManager,
    /// Access to manage corporation fittings
    #[serde(rename = "Fitting_Manager")]
    FittingManager,
    /// View-only access for corporation hangar division #1
    #[serde(rename = "Hangar_Query_1")]
    HangarQuery1,
    /// View-only access for corporation hangar division #2
    #[serde(rename = "Hangar_Query_2")]
    HangarQuery2,
    /// View-only access for corporation hangar division #3
    #[serde(rename = "Hangar_Query_3")]
    HangarQuery3,
    /// View-only access for corporation hangar division #4
    #[serde(rename = "Hangar_Query_4")]
    HangarQuery4,
    /// View-only access for corporation hangar division #5
    #[serde(rename = "Hangar_Query_5")]
    HangarQuery5,
    /// View-only access for corporation hangar division #6
    #[serde(rename = "Hangar_Query_6")]
    HangarQuery6,
    /// View-only access for corporation hangar division #7
    #[serde(rename = "Hangar_Query_7")]
    HangarQuery7,
    /// Ability to remove items from corporation hangar divison #1
    #[serde(rename = "Hangar_Take_1")]
    HangarTake1,
    /// Ability to remove items from corporation hangar divison #2
    #[serde(rename = "Hangar_Take_2")]
    HangarTake2,
    /// Ability to remove items from corporation hangar divison #3
    #[serde(rename = "Hangar_Take_3")]
    HangarTake3,
    /// Ability to remove items from corporation hangar divison #4
    #[serde(rename = "Hangar_Take_4")]
    HangarTake4,
    /// Ability to remove items from corporation hangar divison #5
    #[serde(rename = "Hangar_Take_5")]
    HangarTake5,
    /// Ability to remove items from corporation hangar divison #6
    #[serde(rename = "Hangar_Take_6")]
    HangarTake6,
    /// Ability to remove items from corporation hangar divison #7
    #[serde(rename = "Hangar_Take_7")]
    HangarTake7,
    /// View-only access to wallets, transactions, & assets
    #[serde(rename = "Junior_Accountant")]
    JuniorAccountant,
    /// Ability to invite players and accept applications to the corporation
    #[serde(rename = "Personnel_Manager")]
    PersonnelManager,
    /// Ability to manage corporation projects
    #[serde(rename = "Project_Manager")]
    ProjectManager,
    /// Ability to start manufacturing jobs on behalf of the corporation
    #[serde(rename = "Rent_Factory_Facility")]
    RentFactoryFacility,
    /// Ability to rent corporation offices
    #[serde(rename = "Rent_Office")]
    RentOffice,
    /// Ability to start research jobs on behalf of the corporation
    #[serde(rename = "Rent_Research_Facility")]
    RentResearchFacility,
    /// Can view contents of member's hangars in NPC stations where the corporation owns an
    /// office, does not apply to structures
    #[serde(rename = "Security_Officer")]
    SecurityOfficer,
    /// Can manage corporation skill plans
    #[serde(rename = "Skill_Plan_Manager")]
    SkillPlanManager,
    /// Can take control of POS tower weapons
    #[serde(rename = "Starbase_Defense_Operator")]
    StarbaseDefenseOperator,
    /// Access to the fuel bays & silos of a POS tower
    #[serde(rename = "Starbase_Fuel_Technician")]
    StarbaseFuelTechnician,
    /// Full access to manage & deploy Upwell Structures
    #[serde(rename = "Station_Manager")]
    StationManager,
    /// Full access to corporation deliveries and ability to create market orders on behalf
    /// of the corporation.
    Trader,
}

/// Indicates the type of action on an audit log secure container log entry
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdContainersLogsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CorporationSecureContainerAction {
    /// Added item to container
    #[serde(rename = "add")]
    Add,
    /// Assembled a container
    #[serde(rename = "assemble")]
    Assemble,
    /// Configured a container
    #[serde(rename = "configure")]
    Configure,
    /// Entered password for container
    #[serde(rename = "enter_password")]
    EnterPassword,
    /// Locked container
    #[serde(rename = "lock")]
    Lock,
    /// Moved container
    #[serde(rename = "move")]
    Move,
    /// Repackaged container
    #[serde(rename = "repackage")]
    Repackage,
    /// Set name for container
    #[serde(rename = "set_name")]
    SetName,
    /// Set password for container
    #[serde(rename = "set_password")]
    SetPassword,
    /// Unlocked container
    #[serde(rename = "unlock")]
    Unlock,
}

/// Indicates whether shares are held by a character or corporation
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdShareholdersGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ShareholderType {
    /// Shares are held by a character
    #[serde(rename = "character")]
    Character,
    /// Shares are held by a corporation
    #[serde(rename = "corporation")]
    Corporation,
}

/// Indicates the current state of a corporation starbase (POS)
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStarbasesGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CorporationStarbaseState {
    /// The starbase (POS) is offline
    #[serde(rename = "offline")]
    Offline,
    /// The starbase (POS) is online
    #[serde(rename = "online")]
    Online,
    /// The starbase (POS) is onlining
    #[serde(rename = "onlining")]
    Onlining,
    /// The starbase (POS) is reinforced
    #[serde(rename = "reinforced")]
    Reinforced,
    /// The starbase (POS) is unanchoring
    #[serde(rename = "unanchoring")]
    Unanchoring,
}

/// The permission required to perform an action on a corporation owned starbase (POS)
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStarbasesStarbaseIdGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CorporationStarbasePermission {
    /// Alliance members have permission
    #[serde(rename = "alliance_member")]
    AllianceMember,
    /// Corporation members with `Config Starbase Equipment` role have permission
    #[serde(rename = "config_starbase_equipment_role")]
    ConfigStarbaseEquipmentRole,
    /// Corporation members have permission
    #[serde(rename = "corporation_member")]
    CorporationMember,
    /// Corporation members with `Starbase Fuel Technician` role have permission
    #[serde(rename = "starbase_fuel_technician_role")]
    StarbaseFuelTechnicianRole,
}

/// The possible states of a corporation's Upwell structure's service module
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStructuresGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CorporationStructureServiceState {
    /// Structure service is currently online
    #[serde(rename = "online")]
    Online,
    /// Structure service is currently offline
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "cleanup")]
    Cleanup,
}

/// The possible states of a corporation's Upwell structure
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdStructuresGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CorporationStructureState {
    /// Structure finished anchoring but has not yet had quantum core installed and is currently vulnerable
    #[serde(rename = "anchor_vulnerable")]
    AnchorVulnerable,
    /// Structure is currently anchoring
    #[serde(rename = "anchoring")]
    Anchoring,
    /// Structure's shield has depleted and armor timer is upcoming
    #[serde(rename = "armor_reinforce")]
    ArmorReinforce,
    /// Structure's armor timer has elapsed and armor is now vulnerable
    #[serde(rename = "armor_vulnerable")]
    ArmorVulnerable,
    /// Structure has started anchoring and is currently vulnerable
    #[serde(rename = "deploy_vulnerable")]
    DeployVulnerable,
    #[serde(rename = "fitting_invulnerable")]
    FittingInvulnerable,
    /// Structure's armor has depleted and hull timer is upcoming
    #[serde(rename = "hull_reinforce")]
    HullReinforce,
    /// Structure's hull timer has elapsed and hull is now vulnerable
    #[serde(rename = "hull_vulnerable")]
    HullVulnerable,
    /// Deprecated online state before structures were set to be always vulnerable
    #[serde(rename = "online_deprecated")]
    OnlineDeprecated,
    /// Structure is currently onlining after having quantum core installed
    #[serde(rename = "onlining_vulnerable")]
    OnliningVulnerable,
    /// Structure shield is vulnerable to attack (default state)
    #[serde(rename = "shield_vulnerable")]
    ShieldVulnerable,
    /// Structure has been unanchored
    #[serde(rename = "unanchored")]
    Unanchored,
    #[serde(rename = "unknown")]
    Unknown,
}
