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
    /// Alliance kicked from faction warfare due to low standings
    FWAllianceKickMsg,
    /// Alliance warned for nearing removal from faction warfare due to low standings
    FWAllianceWarningMsg,
    /// Character kicked from faction warfare due to low standings
    FWCharKickMsg,
    /// Character has gained a rank in faction warfare
    FWCharRankGainMsg,
    /// Character has lost a rank in faction warfare
    FWCharRankLossMsg,
    /// Character warned for nearing removal from faction warfare due to low standings
    FWCharWarningMsg,
    /// Corporation has joined faction warfare
    FWCorpJoinMsg,
    /// Corporation has been kicked from faction warfare due to low standings
    FWCorpKickMsg,
    /// Corporation has left faction warfare
    FWCorpLeaveMsg,
    /// Corporation warned for nearing removal from faction warfare due to low standings
    FWCorpWarningMsg,
    /// Corporation requested to join faction warfare
    FacWarCorpJoinRequestMsg,
    /// Corporation has withdrawn join request for faction warfare
    FacWarCorpJoinWithdrawMsg,
    /// Corporation has requested to leave faction warfare
    FacWarCorpLeaveRequestMsg,
    /// Corporation has withdrawn leave request for faction warfare
    FacWarCorpLeaveWithdrawMsg,
    /// Direct enlistment to faction warfare revoked for character
    FacWarDirectEnlistmentRevoked,
    /// Loyalty points for faction warfare disqualified
    FacWarLPDisqualifiedEvent,
    /// Loyalty points for faction warfare kill disqualified
    FacWarLPDisqualifiedKill,
    /// Loyalty points for faction warfare have been paid out
    FacWarLPPayoutEvent,
    /// Loyalty points for kill in faction warfare has been paid out
    FacWarLPPayoutKill,
    /// Freelance project has been closed
    FreelanceProjectClosed,
    /// Freelance project has been completed
    FreelanceProjectCompleted,
    /// Freelance project has been created
    FreelanceProjectCreated,
    /// Freelance project expired
    FreelanceProjectExpired,
    /// Freelance project limit has been reached
    FreelanceProjectLimitReached,
    /// Freelance project participant has been kicked
    FreelanceProjectParticipantKicked,
    /// Omega game time has been added
    GameTimeAdded,
    /// Omega game time has been received
    GameTimeReceived,
    /// Omega game time gift has been sent
    GameTimeSent,
    /// Omega game time gift has been received
    GiftReceived,
    /// Infrastructure hub destroyed due to failure to pay maintenance bills
    IHubDestroyedByBillFailure,
    /// Sansha incursion has completed
    IncursionCompletedMsg,
    /// ???
    IndustryOperationFinished,
    /// ???
    IndustryTeamAuctionLost,
    /// ???
    IndustryTeamAuctionWon,
    /// Infrastructure hub bill is about to expire
    InfrastructureHubBillAboutToExpire,
    /// Insurance policy for ship has expired
    InsuranceExpirationMsg,
    /// Insurance replacement starter ship has been issued
    InsuranceFirstShipMsg,
    /// Insurance policy for ship has been invalidated
    InsuranceInvalidatedMsg,
    /// Insurance policy for ship has been issued
    InsuranceIssuedMsg,
    /// Insurance policy for ship has been paid out
    InsurancePayoutMsg,
    /// Invasion of system has been completed
    InvasionCompletedMsg,
    /// System character is located is currently under invasion
    InvasionSystemLogin,
    /// Invasion of system has started
    InvasionSystemStart,
    /// Character jump clone has been deleted
    JumpCloneDeletedMsg1,
    /// Character jump clone has been deleted
    JumpCloneDeletedMsg2,
    /// Kill report for ship killed now available
    KillReportFinalBlow,
    /// Kill report for ship lost now available
    KillReportVictim,
    /// Kill right now available for ship loss in highsec
    KillRightAvailable,
    /// Kill right has now opened
    KillRightAvailableOpen,
    /// Kill right earned
    KillRightEarned,
    /// Kill right is unavailable
    KillRightUnavailable,
    /// Unavailable kill right has now been opened
    KillRightUnavailableOpen,
    /// Kill right has been used to kill character in highsec
    KillRightUsed,
    /// Loyalty points have been automatically redeemed
    LPAutoRedeemed,
    /// Locator agent has found character
    LocateCharMsg,
    /// War declaration has been made mutual
    MadeWarMutual,
    /// Mercenary offer for war declaration has been retracted
    MercOfferRetractedMsg,
    /// Mercenary offer for war declaration received
    MercOfferedNegotiationMsg,
    /// Merecenary den under attack
    MercenaryDenAttacked,
    /// ???
    MercenaryDenNewMTO,
    /// Mercenary den has been reinforced
    MercenaryDenReinforced,
    /// Mission has been cancelled due to system falling under triglavian control
    MissionCanceledTriglavian,
    /// Mission offer has been expired
    MissionOfferExpirationMsg,
    /// Mission offer has timed out
    MissionTimeoutMsg,
    /// Moon mining extraction for Upwell structure has automatically fractured
    MoonminingAutomaticFracture,
    /// Moon mining extraction for Upwell structure has been cancelled
    MoonminingExtractionCancelled,
    /// Moon mining extraction for Upwell structure has finished
    MoonminingExtractionFinished,
    /// Moon mining extraction for Upwell structure has started
    MoonminingExtractionStarted,
    /// Moon mining laser has been fired for upwell structure
    MoonminingLaserFired,
    /// Mutual war declaration has expired
    MutualWarExpired,
    /// Mutual war declaration invite has been accepted
    MutualWarInviteAccepted,
    /// Mutual war declaration invite has been rejected
    MutualWarInviteRejected,
    /// Mutual war invite has been sent
    MutualWarInviteSent,
    /// NPC standings has been gained
    NPCStandingsGained,
    /// NPC standings has been lost
    NPCStandingsLost,
    /// Offer to ally in war declaration has been retracted
    OfferToAllyRetracted,
    /// Offered to surrender in war declaration
    OfferedSurrender,
    /// Offered to ally in war declaration
    OfferedToAlly,
    /// Corporation office leased cancelled due to insufficient standing with
    /// station owner.
    OfficeLeaseCanceledInsufficientStandings,
    /// ???
    OldLscMessages,
    /// ???
    OperationFinished,
    /// Customs office has been attacked
    OrbitalAttacked,
    /// Customs office has been reinforced
    OrbitalReinforced,
    /// Ownership of structure has been transferred
    OwnershipTransferred,
    /// Raffle has been created
    RaffleCreated,
    /// Raffle expired
    RaffleExpired,
    /// Raffle finished
    RaffleFinished,
    /// ???
    ReimbursementMsg,
    /// Research agent mission now available
    ResearchMissionAvailableMsg,
    /// War declaration has been retracted
    RetractsWar,
    /// Skillpoint reward has been automatically redeemed
    SPAutoRedeemed,
    /// Seasonal challenge has been completed
    SeasonalChallengeCompleted,
    /// Skin sequencing has been completed
    SkinSequencingCompleted,
    /// Planetary skyhook has been deployed
    SkyhookDeployed,
    /// Planetary skyhook has been destroyed
    SkyhookDestroyed,
    /// Planetary skyhook has lost shields
    SkyhookLostShields,
    /// Planetary skyhook is now online
    SkyhookOnline,
    /// Planetary skyhook is under attack
    SkyhookUnderAttack,
    /// Alliance sovreignty over system acquired
    SovAllClaimAquiredMsg,
    /// Alliance sovreignty over system has been lost
    SovAllClaimLostMsg,
    /// Sovreignty command nodes now vulnerable to entosis to contest system ownership
    SovCommandNodeEventStarted,
    /// Corporation sovreignty maintenance bill late
    SovCorpBillLateMsg,
    /// Corporation sovreignty claim failed
    SovCorpClaimFailMsg,
    /// Sovreignty disrupted (old deprecated blockade system?)
    SovDisruptorMsg,
    /// Player station has entered freeport (deprecated stations prior to Upwell)
    SovStationEnteredFreeport,
    /// Sovreignty structure has been destroyed
    SovStructureDestroyed,
    /// Sovreignty structure has been reinforced
    SovStructureReinforced,
    /// Sovreignty structure self-destruct has been cancelled
    SovStructureSelfDestructCancel,
    /// Sovreignty structure self-destruct finished
    SovStructureSelfDestructFinished,
    /// Sovreignty stucture self-destruct has been initiated
    SovStructureSelfDestructRequested,
    /// Sovreignty infrastructure has been damaged (old sov mechanics?)
    SovereigntyIHDamageMsg,
    /// ???
    SovereigntySBUDamageMsg,
    /// Sovreignty territorial claim unit has been damaged (old sov mechanics?)
    SovereigntyTCUDamageMsg,
    /// Player station has been attacked (deprecated stations prior to upwell)
    StationAggressionMsg1,
    /// Player station has been attacked (deprecated stations prior to upwell)
    StationAggressionMsg2,
    /// Player station has been conquered (deprecated stations prior to upwell)
    StationConquerMsg,
    /// Player station service has been disabled (deprecated stations prior to upwell)
    StationServiceDisabled,
    /// Player station service has been enabled (deprecated stations prior to upwell)
    StationServiceEnabled,
    /// Player station state has changed (deprecated stations prior to upwell)
    StationStateChangeMsg,
    /// Storyline mission is now available
    StoryLineMissionAvailableMsg,
    /// Upwell structure has begun anchoring
    StructureAnchoring,
    /// Upwell structure courier contract has been changed
    StructureCourierContractChanged,
    /// Upwell structure has been destroyed
    StructureDestroyed,
    /// Upwell structre low fuel notification
    StructureFuelAlert,
    /// Upwell structure has gone low power and is pending abandonment, assets may be dropped
    /// if structure is destroyed while abandoned.
    StructureImpendingAbandonmentAssetsAtRisk,
    /// Structure items in asset safety have been delivered
    StructureItemsDelivered,
    /// Structure items have been moved to asset safety
    StructureItemsMovedToSafety,
    /// Upwell structure has lost armor
    StructureLostArmor,
    /// Upwell structure has lost shields
    StructureLostShields,
    /// Upwell sovreignty hub is low on reagents
    StructureLowReagentsAlert,
    /// Upwell sovreignty hub is out of reagents
    StructureNoReagentsAlert,
    /// Upwell structure is now online
    StructureOnline,
    /// Upwell structure skin has been purchased
    StructurePaintPurchased,
    /// Upwell structure services are now offline
    StructureServicesOffline,
    /// Upwell structure is now unanchoring
    StructureUnanchoring,
    /// Upwell structure is under attack
    StructureUnderAttack,
    /// Upwell structure is now in high power
    StructureWentHighPower,
    /// Upwell structure is now in low power
    StructureWentLowPower,
    /// Upwell structure industry/research jobs have been cancelled
    StructuresJobsCancelled,
    /// Upwell structure industry/research jobs have been paused
    StructuresJobsPaused,
    /// Upwell structure reinforcement time has changed
    StructuresReinforcementChanged,
    /// POS tower-related alerts
    TowerAlertMsg,
    /// POS tower running low on resources
    TowerResourceAlertMsg,
    /// ???
    TransactionReversalMsg,
    /// Starter tutorial message
    TutorialMsg,
    /// War declaration has been inherited from corporation joining alliance
    WarAdopted,
    /// War declaration ally has been inherited from corporation joining alliance
    WarAllyInherited,
    /// War declaration ally offer has been declined
    WarAllyOfferDeclinedMsg,
    /// War declaration has been invalidated by CONCORD
    WarConcordInvalidates,
    /// War has been declared against corporation/alliance
    WarDeclared,
    /// War ended due to war aggressor HQ's system security status dropping below 0.5
    WarEndedHqSecurityDrop,
    /// War ended due to war aggressor HQ being destroyed
    WarHQRemovedFromSpace,
    /// War declaration has been inherited from corporation joining alliance
    WarInherited,
    /// War declaration has been invalidated
    WarInvalid,
    /// War declaration has been retracted
    WarRetracted,
    /// War declaration has been retracted by CONCORD
    WarRetractedByConcord,
    /// War surrender offer has been declined
    WarSurrenderDeclinedMsg,
    /// War surrender offer has been received
    WarSurrenderOfferMsg,
}
