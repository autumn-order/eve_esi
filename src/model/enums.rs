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

/// Indicates the type of notification
///
/// Note: most fields have documentation, if they are documented with `???` then it is unknown
/// what exactly this location field actually applies to in-game.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdNotificationsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NotificationType {
    AcceptedAlly,
    AcceptedSurrender,
    AgentRetiredTrigravian,
    AllAnchoringMsg,
    AllMaintenanceBillMsg,
    AllStrucInvulnerableMsg,
    AllStructVulnerableMsg,
    AllWarCorpJoinedAllianceMsg,
    AllWarDeclaredMsg,
    AllWarInvalidatedMsg,
    AllWarRetractedMsg,
    AllWarSurrenderMsg,
    AllianceCapitalChanged,
    AllianceWarDeclaredV2,
    AllyContractCancelled,
    AllyJoinedWarAggressorMsg,
    AllyJoinedWarAllyMsg,
    AllyJoinedWarDefenderMsg,
    BattlePunishFriendlyFire,
    BillOutOfMoneyMsg,
    BillPaidCorpAllMsg,
    BountyClaimMsg,
    BountyESSShared,
    BountyESSTaken,
    BountyPlacedAlliance,
    BountyPlacedChar,
    BountyPlacedCorp,
    BountyYourBountyClaimed,
    BuddyConnectContactAdd,
    CharAppAcceptMsg,
    CharAppRejectMsg,
    CharAppWithdrawMsg,
    CharLeftCorpMsg,
    CharMedalMsg,
    CharTerminationMsg,
    CloneActivationMsg,
    CloneActivationMsg2,
    CloneMovedMsg,
    CloneRevokedMsg1,
    CloneRevokedMsg2,
    CombatOperationFinished,
    ContactAdd,
    ContactEdit,
    ContainerPasswordMsg,
    ContractRegionChangedToPochven,
    CorpAllBillMsg,
    CorpAppAcceptMsg,
    CorpAppInvitedMsg,
    CorpAppNewMsg,
    CorpAppRejectCustomMsg,
    CorpAppRejectMsg,
    CorpBecameWarEligible,
    CorpDividendMsg,
    CorpFriendlyFireDisableTimerCompleted,
    CorpFriendlyFireDisableTimerStarted,
    CorpFriendlyFireEnableTimerCompleted,
    CorpFriendlyFireEnableTimerStarted,
    CorpKicked,
    CorpLiquidationMsg,
    CorpNewCEOMsg,
    CorpNewsMsg,
    CorpNoLongerWarEligible,
    CorpOfficeExpirationMsg,
    CorpStructLostMsg,
    CorpTaxChangeMsg,
    CorpVoteCEORevokedMsg,
    CorpVoteMsg,
    CorpWarDeclaredMsg,
    CorpWarDeclaredV2,
    CorpWarFightingLegalMsg,
    CorpWarInvalidatedMsg,
    CorpWarRetractedMsg,
    CorpWarSurrenderMsg,
    CorporationGoalClosed,
    CorporationGoalCompleted,
    CorporationGoalCreated,
    CorporationGoalExpired,
    CorporationGoalLimitReached,
    CorporationGoalNameChange,
    CorporationLeft,
    CustomsMsg,
    DailyItemRewardAutoClaimed,
    DeclareWar,
    DistrictAttacked,
    DustAppAcceptedMsg,
    ESSMainBankLink,
    EntosisCaptureStarted,
    ExpertSystemExpired,
    ExpertSystemExpiryImminent,
    FWAllianceKickMsg,
    FWAllianceWarningMsg,
    FWCharKickMsg,
    FWCharRankGainMsg,
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
