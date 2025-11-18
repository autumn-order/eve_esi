//! # EVE ESI Notification Enums
//!
//! Provides enums related to notifications in EVE Online
//!
//! ## Enums
//! - [`NotificationSenderType`]: The type of sender for a notification
//! - [`NotificationType`]: Indicates the type of notification

use serde::{Deserialize, Serialize};

/// The type of sender for a notification
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdNotificationsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NotificationSenderType {
    /// Notification was sent by character
    #[serde(rename = "character")]
    Character,
    /// Notification was sent by corporation
    #[serde(rename = "corporation")]
    Corporation,
    /// Notification was sent by alliance
    #[serde(rename = "alliance")]
    Alliance,
    /// Notification was sent by faction
    #[serde(rename = "faction")]
    Faction,
    /// Notification was sent by other
    #[serde(rename = "other")]
    Other,
}

/// Indicates the type of notification
///
/// Note: All fields have documentation, the majority of it should be correct, if you find that something isn't
/// please submit a pull request to fix it. If a field is documented with `???` then it is unknown what exactly this
/// notification enum variant is actually for.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdNotificationsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    // ESI returns a space in this variant for some reason
    #[serde(rename = "WarAdopted ")]
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
