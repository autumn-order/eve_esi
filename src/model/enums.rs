//! # EVE ESI Model Enums
//!
//! Defines shared enums used across multiple ESI models.
//!
//! # Enums
//! - [`LocationFlag`]: Indicates where an item, module, or ship in EVE Online is located

use serde::{Deserialize, Serialize};

/// Indicates where an item, module, or ship in EVE Online is located
///
/// This enum represents the location of items, ships, and modules returned by primarily assets
/// endpoints and other item or ship-related ESI endpoints
///
/// Note: most fields have documentation, if they are documented with `???` then it is unknown
/// what exactly this location field actually applies to in-game.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdBlueprintsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum LocationFlag {
    /// ???
    AutoFit,
    /// Ship's cargo bay
    Cargo,
    /// Specialized corpse bay (Blood Raider ships)
    CorpseBay,
    /// Ship's drone bay
    DroneBay,
    /// Ship's fleet hangar
    FleetHangar,
    /// Station deliveries hangar
    Deliveries,
    /// ???
    HiddenModifiers,
    /// Station hangar
    Hangar,
    /// Station hangar?
    HangarAll,
    /// Ship or structure's low slot #1
    LoSlot0,
    /// Ship or structure's low slot #2
    LoSlot1,
    /// Ship or structure's low slot #3
    LoSlot2,
    /// Ship or structure's low slot #4
    LoSlot3,
    /// Ship or structure's low slot #5
    LoSlot4,
    /// Ship or structure's low slot #6
    LoSlot5,
    /// Ship or structure's low slot #7
    LoSlot6,
    /// Ship or structure's low slot #8
    LoSlot7,
    /// Ship or structure's middle slot #1
    MedSlot0,
    /// Ship or structure's middle slot #2
    MedSlot1,
    /// Ship or structure's middle slot #3
    MedSlot2,
    /// Ship or structure's middle slot #4
    MedSlot3,
    /// Ship or structure's middle slot #5
    MedSlot4,
    /// Ship or structure's middle slot #6
    MedSlot5,
    /// Ship or structure's middle slot #7
    MedSlot6,
    /// Ship or structure's middle slot #8
    MedSlot7,
    /// Ship or structure's high slot #1
    HiSlot0,
    /// Ship or structure's high slot #2
    HiSlot1,
    /// Ship or structure's high slot #3
    HiSlot2,
    /// Ship or structure's high slot #4
    HiSlot3,
    /// Ship or structure's high slot #5
    HiSlot4,
    /// Ship or structure's high slot #6
    HiSlot5,
    /// Ship or structure's high slot #7
    HiSlot6,
    /// Ship or structure's high slot #8
    HiSlot7,
    /// Station asset safety storage
    AssetSafety,
    /// ???
    Locked,
    /// ???
    Unlocked,
    /// Implant slot on a character
    Implant,
    /// Ship's specialized quafe storage bay
    QuafeBay,
    /// Ship or structure's rig slot #1
    RigSlot0,
    /// Ship or structure's rig slot #2
    RigSlot1,
    /// Ship or structure's rig slot #3
    RigSlot2,
    /// Ship or structure's rig slot #4
    RigSlot3,
    /// Ship or structure's rig slot #5
    RigSlot4,
    /// Ship or structure's rig slot #6
    RigSlot5,
    /// Ship or structure's rig slot #7
    RigSlot6,
    /// Ship or structure's rig slot #8
    RigSlot7,
    /// Station's ship hangar storage bay
    ShipHangar,
    /// Ship's specialized fuel storage bay
    SpecializedFuelBay,
    /// Ship's specialized ore storage bay
    SpecializedOreHold,
    /// Ship's specialized gas storage bay
    SpecializedGasHold,
    /// Ship's specialized mineral storage bay
    SpecializedMineralHold,
    /// Ship's specialized salvage storage bay
    SpecializedSalvageHold,
    /// Ship's specialized ship storage bay
    SpecializedShipHold,
    /// Ship's specialized small ship storage bay
    SpecializedSmallShipHold,
    /// Ship's specialized medium ship storage bay
    SpecializedMediumShipHold,
    /// Ship's specialized large ship storage bay
    SpecializedLargeShipHold,
    /// Ship's specialized industrial ship storage bay
    SpecializedIndustrialShipHold,
    /// Ship's specialized ammo storage bay
    SpecializedAmmoHold,
    /// Ship's specialized command center storage bay (Epithal, Primae)
    SpecializedCommandCenterHold,
    /// Ship's specialized command center storage bay (Epithal, Primae)
    SpecializedPlanetaryCommoditiesHold,
    /// Ship's specialized material bay
    SpecializedMaterialBay,
    /// Ship's subsystem slot #1 (T3 cruisers)
    SubSystemSlot0,
    /// Ship's subsystem slot #2 (T3 cruisers)
    SubSystemSlot1,
    /// Ship's subsystem slot #3 (T3 cruisers)
    SubSystemSlot2,
    /// Ship's subsystem slot #4 (T3 cruisers)
    SubSystemSlot3,
    /// Ship's subsystem slot #5 (T3 cruisers)
    SubSystemSlot4,
    /// Ship's subsystem slot #6 (T3 cruisers)
    SubSystemSlot5,
    /// Ship's subsystem slot #7 (T3 cruisers)
    SubSystemSlot6,
    /// Ship's subsystem slot #8 (T3 cruisers)
    SubSystemSlot7,
    /// Carrier or structure's fighter bay
    FighterBay,
    /// Carrier or structure's fighter tube #1
    FighterTube0,
    /// Carrier or structure's fighter tube #2
    FighterTube1,
    /// Carrier or structure's fighter tube #3
    FighterTube2,
    /// Carrier or structure's fighter tube #4
    FighterTube3,
    /// Carrier or structure's fighter tube #5
    FighterTube4,
    /// ???
    Module,
}

/// Indicates the type of corporation role
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdRolesGet>
/// - <https://support.eveonline.com/hc/en-us/articles/203217712-Roles-Listing>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

/// Indicates the type of notification
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdNotificationsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NotificationType {
    /// Accepted alliance request for corporation war
    AcceptedAlly,
    /// Accepted surrender for corporation war
    AcceptedSurrender,
    /// Agent retired due to system falling under triglavian control
    AgentRetiredTrigravian,
    /// Structure anchoring in alliance space
    AllAnchoringMsg,
    /// Alliance maintenance bill-related message
    AllMaintenanceBillMsg,
    /// Alliance structure invulnerable
    AllStrucInvulnerableMsg,
    /// Alliance structure vulnerable
    AllStructVulnerableMsg,
    /// Corporation at war joined the alliance
    AllWarCorpJoinedAllianceMsg,
    /// War has been declared involving character's alliance
    AllWarDeclaredMsg,
    /// War involving character's alliance has been invalidated
    AllWarInvalidatedMsg,
    /// War involving character's alliance has been retracted
    AllWarRetractedMsg,
    /// War involving character's alliance has surrendered
    AllWarSurrenderMsg,
    /// Alliance capital system has been changed
    AllianceCapitalChanged,
    /// V2 War has been declared involving character's alliance
    AllianceWarDeclaredV2,
    /// Contract involving character's alliance has been cancelled
    AllyContractCancelled,
    /// War aggressor has had an ally join the war
    AllyJoinedWarAggressorMsg,
    /// Ally has joined war against aggressor
    AllyJoinedWarAllyMsg,
    /// Ally has joined war as a defender
    AllyJoinedWarDefenderMsg,
    /// Punishment for friendly fire
    BattlePunishFriendlyFire,
    /// Unable to pay corporation bill due to lack of funds
    BillOutOfMoneyMsg,
    /// Corporation bill has been paid
    BillPaidCorpAllMsg,
    /// Bounty on player has been claimed
    BountyClaimMsg,
    /// Funds within ESS have been paid out
    BountyESSShared,
    /// Funds within ESS have been stolen
    BountyESSTaken,
    /// Bounty has been placed on alliance
    BountyPlacedAlliance,
    /// Bounty has been placed on character
    BountyPlacedChar,
    /// Bounty has been placed on corporation
    BountyPlacedCorp,
    /// Bounty on notification receiver has been claimed
    BountyYourBountyClaimed,
    /// Contact has been added as buddy
    BuddyConnectContactAdd,
    /// Character application to corporation has been accepted
    CharAppAcceptMsg,
    /// Character application to corporation has been rejected
    CharAppRejectMsg,
    /// Character application to corporation has been withdrawn
    CharAppWithdrawMsg,
    /// Character has left corporation
    CharLeftCorpMsg,
    /// Character has received a medal from corporation
    CharMedalMsg,
    /// Character in corporation has been biomassed
    CharTerminationMsg,
    /// Clone has been activated (capsule death/jump clone)
    CloneActivationMsg,
    /// Clone has been activated (capsule death/jump clone)
    CloneActivationMsg2,
    /// Character's clone has been moved
    CloneMovedMsg,
    /// Character's clone has been revoked (lost access to station facilities)
    CloneRevokedMsg1,
    /// Character's clone has been revoked (lost access to station facilities)
    CloneRevokedMsg2,
    /// ???
    CombatOperationFinished,
    /// Character notified they have been added as contact
    ContactAdd,
    /// Character notified their contact info has been edited
    ContactEdit,
    /// ???
    ContainerPasswordMsg,
    /// Contract was moved to Pochven due to system falling under Triglavian control
    ContractRegionChangedToPochven,
    /// Corporation alliance bill notification
    CorpAllBillMsg,
    /// Applicant to the corporation has been accepted
    CorpAppAcceptMsg,
    /// Applicant has been invited to join corporation
    CorpAppInvitedMsg,
    /// Corporation has received new character application
    CorpAppNewMsg,
    /// Applicant to the corporation has been rejected with custom message
    CorpAppRejectCustomMsg,
    /// Applicant to the corporation has been rejected with default message
    CorpAppRejectMsg,
    /// Corporation has become war eligible
    CorpBecameWarEligible,
    /// Corporation dividend has been paid out
    CorpDividendMsg,
    /// Corporation friendly fire disabled is now active
    CorpFriendlyFireDisableTimerCompleted,
    /// Corporation friendly fire disabled countdown has started
    CorpFriendlyFireDisableTimerStarted,
    /// Croporation friendly fire enabled is now active
    CorpFriendlyFireEnableTimerCompleted,
    /// Corporation friendly fire enabled countdown has started
    CorpFriendlyFireEnableTimerStarted,
    /// Corporation has been kicked from alliance
    CorpKicked,
    /// Corporation has been liquidated due to being disbanded
    CorpLiquidationMsg,
    /// Corporation has new CEO
    CorpNewCEOMsg,
    /// ???
    CorpNewsMsg,
    /// Corporation is not longer war eligible
    CorpNoLongerWarEligible,
    /// Corporation office has expired
    CorpOfficeExpirationMsg,
    /// Corporation has lost a structure
    CorpStructLostMsg,
    /// Corporation tax rate has been changed
    CorpTaxChangeMsg,
    /// Corporation change CEO vote has been revoked
    CorpVoteCEORevokedMsg,
    /// Corporation change CEO vote has been started
    CorpVoteMsg,
    /// Corporation has declared war
    CorpWarDeclaredMsg,
    /// V2 corporation has declared war
    CorpWarDeclaredV2,
    /// Corporaiton war declaration is now active
    CorpWarFightingLegalMsg,
    /// Corporation's war declaration has been invalidated
    CorpWarInvalidatedMsg,
    /// Corporation's war declaration has been retracted
    CorpWarRetractedMsg,
    /// Corporation has surrendered in war declaration
    CorpWarSurrenderMsg,
    /// Corporation projects goal has been closed
    CorporationGoalClosed,
    /// Corporation projects goal completed
    CorporationGoalCompleted,
    /// Corporation projects goal has been created
    CorporationGoalCreated,
    /// Corporation projects goal has expired
    CorporationGoalExpired,
    /// Corporation projects goal limit has been reached
    CorporationGoalLimitReached,
    /// Corporation projects goal name has been changed
    CorporationGoalNameChange,
    /// Corporation has left alliance
    CorporationLeft,
    /// ???
    CustomsMsg,
    /// Daily reward item has been automatically claimed
    DailyItemRewardAutoClaimed,
    /// ???
    DeclareWar,
    /// ???
    DistrictAttacked,
    /// ???
    DustAppAcceptedMsg,
    /// ESS main bank has been linked with
    ESSMainBankLink,
    /// Entosis capture of sovreignty hub has started
    EntosisCaptureStarted,
    /// Expert system has expired
    ExpertSystemExpired,
    /// Expert system is nearing expiry
    ExpertSystemExpiryImminent,
    /// Alliance kicked from faction warfare due to standings
    FWAllianceKickMsg,
    /// Alliance warned for nearing removal from faction warfare due to standings
    FWAllianceWarningMsg,
    /// Character kicked from faction warfare due to standings
    FWCharKickMsg,
    /// Character has gained a rank in faction warfare
    FWCharRankGainMsg,
    /// Character has lost a rank in faction warfare
    FWCharRankLossMsg,
    FWCharWarningMsg,
    FWCorpJoinMsg,
    FWCorpKickMsg,
    FWCorpLeaveMsg,
    FWCorpWarningMsg,
    FacWarCorpJoinRequestMsg,
    FacWarCorpJoinWithdrawMsg,
    FacWarCorpLeaveRequestMsg,
    FacWarCorpLeaveWithdrawMsg,
    FacWarDirectEnlistmentRevoked,
    FacWarLPDisqualifiedEvent,
    FacWarLPDisqualifiedKill,
    FacWarLPPayoutEvent,
    FacWarLPPayoutKill,
    FreelanceProjectClosed,
    FreelanceProjectCompleted,
    FreelanceProjectCreated,
    FreelanceProjectExpired,
    FreelanceProjectLimitReached,
    FreelanceProjectParticipantKicked,
    GameTimeAdded,
    GameTimeReceived,
    GameTimeSent,
    GiftReceived,
    IHubDestroyedByBillFailure,
    IncursionCompletedMsg,
    IndustryOperationFinished,
    IndustryTeamAuctionLost,
    IndustryTeamAuctionWon,
    InfrastructureHubBillAboutToExpire,
    InsuranceExpirationMsg,
    InsuranceFirstShipMsg,
    InsuranceInvalidatedMsg,
    InsuranceIssuedMsg,
    InsurancePayoutMsg,
    InvasionCompletedMsg,
    InvasionSystemLogin,
    InvasionSystemStart,
    JumpCloneDeletedMsg1,
    JumpCloneDeletedMsg2,
    KillReportFinalBlow,
    KillReportVictim,
    KillRightAvailable,
    KillRightAvailableOpen,
    KillRightEarned,
    KillRightUnavailable,
    KillRightUnavailableOpen,
    KillRightUsed,
    LPAutoRedeemed,
    LocateCharMsg,
    MadeWarMutual,
    MercOfferRetractedMsg,
    MercOfferedNegotiationMsg,
    MercenaryDenAttacked,
    MercenaryDenNewMTO,
    MercenaryDenReinforced,
    MissionCanceledTriglavian,
    MissionOfferExpirationMsg,
    MissionTimeoutMsg,
    MoonminingAutomaticFracture,
    MoonminingExtractionCancelled,
    MoonminingExtractionFinished,
    MoonminingExtractionStarted,
    MoonminingLaserFired,
    MutualWarExpired,
    MutualWarInviteAccepted,
    MutualWarInviteRejected,
    MutualWarInviteSent,
    NPCStandingsGained,
    NPCStandingsLost,
    OfferToAllyRetracted,
    OfferedSurrender,
    OfferedToAlly,
    OfficeLeaseCanceledInsufficientStandings,
    OldLscMessages,
    OperationFinished,
    OrbitalAttacked,
    OrbitalReinforced,
    OwnershipTransferred,
    RaffleCreated,
    RaffleExpired,
    RaffleFinished,
    ReimbursementMsg,
    ResearchMissionAvailableMsg,
    RetractsWar,
    SPAutoRedeemed,
    SeasonalChallengeCompleted,
    SkinSequencingCompleted,
    SkyhookDeployed,
    SkyhookDestroyed,
    SkyhookLostShields,
    SkyhookOnline,
    SkyhookUnderAttack,
    SovAllClaimAquiredMsg,
    SovAllClaimLostMsg,
    SovCommandNodeEventStarted,
    SovCorpBillLateMsg,
    SovCorpClaimFailMsg,
    SovDisruptorMsg,
    SovStationEnteredFreeport,
    SovStructureDestroyed,
    SovStructureReinforced,
    SovStructureSelfDestructCancel,
    SovStructureSelfDestructFinished,
    SovStructureSelfDestructRequested,
    SovereigntyIHDamageMsg,
    SovereigntySBUDamageMsg,
    SovereigntyTCUDamageMsg,
    StationAggressionMsg1,
    StationAggressionMsg2,
    StationConquerMsg,
    StationServiceDisabled,
    StationServiceEnabled,
    StationStateChangeMsg,
    StoryLineMissionAvailableMsg,
    StructureAnchoring,
    StructureCourierContractChanged,
    StructureDestroyed,
    StructureFuelAlert,
    StructureImpendingAbandonmentAssetsAtRisk,
    StructureItemsDelivered,
    StructureItemsMovedToSafety,
    StructureLostArmor,
    StructureLostShields,
    StructureLowReagentsAlert,
    StructureNoReagentsAlert,
    StructureOnline,
    StructurePaintPurchased,
    StructureServicesOffline,
    StructureUnanchoring,
    StructureUnderAttack,
    StructureWentHighPower,
    StructureWentLowPower,
    StructuresJobsCancelled,
    StructuresJobsPaused,
    StructuresReinforcementChanged,
    TowerAlertMsg,
    TowerResourceAlertMsg,
    TransactionReversalMsg,
    TutorialMsg,
    WarAdopted,
    WarAllyInherited,
    WarAllyOfferDeclinedMsg,
    WarConcordInvalidates,
    WarDeclared,
    WarEndedHqSecurityDrop,
    WarHQRemovedFromSpace,
    WarInherited,
    WarInvalid,
    WarRetracted,
    WarRetractedByConcord,
    WarSurrenderDeclinedMsg,
    WarSurrenderOfferMsg,
}
