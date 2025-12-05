//! # EVE ESI Corporation Endpoints
//!
//! This module provides the [`CorporationEndpoints`] struct and associated methods for accessing
//! corporation-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (22)
//! ### Public (4)
//! |                        Endpoint                       |                                Description                                   |
//! | ----------------------------------------------------- | ---------------------------------------------------------------------------- |
//! | [`CorporationEndpoints::get_npc_corporations`]        | Fetches a list of all NPC corporation IDs in EVE Online                      |
//! | [`CorporationEndpoints::get_corporation_information`] | Fetches a corporation's public information from ESI using the corporation ID |
//! | [`CorporationEndpoints::get_alliance_history`]        | Fetches a corporation's alliance history using the provided corporation ID   |
//! | [`CorporationEndpoints::get_corporation_icon`]        | Fetches a corporation's icon using the provided corporation ID               |
//!
//! ### Authenticated (18)
//! |                           Endpoint                             |                                          Description                                              |
//! | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
//! | [`CorporationEndpoints::get_corporation_blueprints`]           | Fetches a list of blueprint entries for the provided corporation ID                               |
//! | [`CorporationEndpoints::get_all_corporation_alsc_logs`]        | Fetches audit log secure container (ALSC) log entries for the provided corporation ID             |
//! | [`CorporationEndpoints::get_corporation_divisions`]            | Fetches a list of hangar & wallet divisions for the provided corporation ID                       |
//! | [`CorporationEndpoints::get_corporation_facilities`]           | Fetches a list of industry facilities for the provided corporation ID                             |
//! | [`CorporationEndpoints::get_corporation_medals`]               | Fetches a paginated list of medals for the provided corporation ID                                |
//! | [`CorporationEndpoints::get_corporation_issued_medals`]        | Fetches a paginated list of issued medals for the provided corporation ID                         |
//! | [`CorporationEndpoints::get_corporation_members`]              | Fetches a list of character IDs of all members part of the provided corporation ID                |
//! | [`CorporationEndpoints::get_corporation_member_limit`]         | Fetches the member limit of the provided corporation ID                                           |
//! | [`CorporationEndpoints::get_corporation_members_titles`]       | Fetches a list of title IDs for each member of the provided corporation ID                        |
//! | [`CorporationEndpoints::track_corporation_members`]            | Fetches a list of tracking information for each character part of the provided corporation ID     |
//! | [`CorporationEndpoints::get_corporation_member_roles`]         | Fetches a list of roles for each character part of the provided corporation ID                    |
//! | [`CorporationEndpoints::get_corporation_member_roles_history`] | Retrieves a paginated list of up to a month of role history for the provided corporation ID       |
//! | [`CorporationEndpoints::get_corporation_shareholders`]         | Retrieves a paginated list of shareholders for the provided corporation ID                        |
//! | [`CorporationEndpoints::get_corporation_standings`]            | Retrieves a paginated list of NPC standing entries for the provided corporation ID                |
//! | [`CorporationEndpoints::get_corporation_starbases`]            | Retrieves a paginated list of starbases (POSes) for the provided corporation ID                   |
//! | [`CorporationEndpoints::get_starbase_detail`]                  | Retrieves details for a starbase (POS) for the provided starbase ID & corporation ID              |
//! | [`CorporationEndpoints::get_corporation_structures`]           | Retrieves a paginated list of structure information for the provided corporation ID               |
//! | [`CorporationEndpoints::get_corporation_titles`]               | Retrieves a list of corporation titles and their respective roles for the provided corporation ID |

use crate::esi::EsiRequest;
use crate::model::asset::Blueprint;
use crate::model::corporation::{
    Corporation, CorporationAllianceHistory, CorporationDivisions, CorporationFacilities,
    CorporationIcon, CorporationIssuedMedal, CorporationMedal, CorporationMemberRoles,
    CorporationMemberRolesHistory, CorporationMemberTitles, CorporationMemberTracking,
    CorporationSecureContainerLog, CorporationShareholder, CorporationStarbase,
    CorporationStarbaseDetails, CorporationStructure, CorporationTitle,
};
use crate::model::standing::Standing;
use crate::scope::{CorporationsScopes, WalletScopes};
use crate::{Client, ScopeBuilder};
use reqwest::Method;

/// Provides methods for accessing corporation-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct CorporationEndpoints<'a> {
    client: &'a Client,
}

impl<'a> CorporationEndpoints<'a> {
    /// Creates a new instance of [`CorporationEndpoints`].
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_esi_endpoint! {
        /// Fetches a list of all NPC corporation IDs in EVE Online
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsNpccorps>
        ///
        /// # Returns
        /// An ESI request builder that returns a list of all NPC corporation IDs when sent.
        pub fn get_npc_corporations(
        ) -> EsiRequest<Vec<i64>>
        method = Method::GET;
        url = "{}/corporations/npccorps";
    }

    define_esi_endpoint! {
        /// Fetches a corporation's public information from ESI using the corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationId>
        ///
        /// # Arguments
        /// - `corporation_id` ([`i64`]): The ID of the corporation to retrieve information for.
        ///
        /// # Returns
        /// An ESI request builder that returns the corporation's public information when sent.
        pub fn get_corporation_information(
            corporation_id: i64
        ) -> EsiRequest<Corporation>
        method = Method::GET;
        url = "{}/corporations/{}";
    }

    define_esi_endpoint! {
        /// Fetches a corporation's alliance history using the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdAlliancehistory>
        ///
        /// # Arguments
        /// - `corporation_id` ([`i64`]): The ID of the corporation to retrieve alliance history for.
        ///
        /// # Returns
        /// An ESI request builder that returns the corporation's alliance history when sent.
        pub fn get_alliance_history(
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationAllianceHistory>>
        method = Method::GET;
        url = "{}/corporations/{}/alliancehistory";
    }

    define_esi_endpoint! {
        /// Fetches a list of blueprint entries for the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdBlueprints>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_blueprints`](crate::scope::CorporationsScopes::read_blueprints):
        ///   `esi-corporations.read_blueprints.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve blueprints for.
        /// - `page`            (`i32`): The page of blueprints to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of blueprint entries for the corporation when sent.
        auth fn get_corporation_blueprints(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<Blueprint>>
        method = Method::GET;
        url = "{}/corporations/{}/blueprints";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_blueprints()).build();
    }

    define_esi_endpoint! {
        /// Fetches audit log secure container (ALSC) log entries for the provided corporation ID
        ///
        /// Contains log information for up to the past 7 days.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdContainersLogs>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_container_logs`](crate::scope::CorporationsScopes::read_container_logs):
        ///   `esi-corporations.read_container_logs.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve ALSC logs for.
        /// - `page`            (`i32`): The page of ALSC logs to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of audit log secure container entries for the corporation when sent.
        auth fn get_all_corporation_alsc_logs(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationSecureContainerLog>>
        method = Method::GET;
        url = "{}/corporations/{}/containers/logs";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_container_logs()).build();
    }

    define_esi_endpoint! {
        /// Fetches a list of hangar & wallet divisions for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdDivisions>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_divisions`](crate::scope::CorporationsScopes::read_divisions):
        ///   `esi-corporations.read_divisions.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve divisions for
        ///
        /// # Returns
        /// An ESI request builder that returns hangar and wallet divisions for the corporation when sent.
        auth fn get_corporation_divisions(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<CorporationDivisions>
        method = Method::GET;
        url = "{}/corporations/{}/divisions";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_divisions()).build();
    }

    define_esi_endpoint! {
        /// Fetches a list of industry facilities for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Factory_Manager` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdFacilities>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_facilities`](crate::scope::CorporationsScopes::read_facilities):
        ///   `esi-corporations.read_facilities.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve facilities for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of industry facilities for the corporation when sent.
        auth fn get_corporation_facilities(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationFacilities>>
        method = Method::GET;
        url = "{}/corporations/{}/facilities";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_facilities()).build();
    }

    define_esi_endpoint! {
        /// Fetches a corporation's icon using the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdIcons>
        ///
        /// # Arguments
        /// - `corporation_id` ([`i64`]): The ID of the corporation to retrieve the icons for
        ///
        /// # Returns
        /// An ESI request builder that returns the corporation's icon URLs when sent.
        pub fn get_corporation_icon(
            corporation_id: i64
        ) -> EsiRequest<CorporationIcon>
        method = Method::GET;
        url = "{}/corporations/{}/icons";
    }

    define_esi_endpoint! {
        /// Fetches a paginated list of medals for the provided corporation ID
        ///
        /// This endpoint differs from [`Self::get_corporation_issued_medals`] in that it describes the medal itself
        /// while [`Self::get_corporation_issued_medals`] represents who issued the medal and who the medal was issued to.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMedals>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_medals`](crate::scope::CorporationsScopes::read_medals):
        ///   `esi-corporations.read_medals.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve medals for
        /// - `page`            (`i32`): The page of medals to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of medal definitions for the corporation when sent.
        auth fn get_corporation_medals(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationMedal>>
        method = Method::GET;
        url = "{}/corporations/{}/medals";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_medals()).build();
    }

    define_esi_endpoint! {
        /// Fetches a paginated list of issued medals for the provided corporation ID
        ///
        /// This endpoint differs from [`Self::get_corporation_medals`] in that it represents who issued the medal
        /// and who the medal was issued to, while [`Self::get_corporation_medals`] describes the medal itself.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMedalsIssued>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_medals`](crate::scope::CorporationsScopes::read_medals):
        ///   `esi-corporations.read_medals.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve issued medals for
        /// - `page`            (`i32`): The page of issued medals to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of issued medal records for the corporation when sent.
        auth fn get_corporation_issued_medals(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationIssuedMedal>>
        method = Method::GET;
        url = "{}/corporations/{}/medals/issued";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_medals()).build();
    }

    define_esi_endpoint! {
        /// Fetches a list of character IDs of all members part of the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must have a specific role
        /// (`Director` or `Personnel_Manager`) in the corporation in order to access this data.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMembers>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_corporation_membership`](crate::scope::CorporationsScopes::read_corporation_membership):
        ///   `esi-corporations.read_corporation_membership.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve members for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of character IDs of corporation members when sent.
        auth fn get_corporation_members(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<i64>>
        method = Method::GET;
        url = "{}/corporations/{}/members";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_corporation_membership())
            .build();
    }

    define_esi_endpoint! {
        /// Fetches the member limit of the provided corporation ID
        ///
        /// Useful for determining how many more members a corporation can have before it reaches capacity.
        ///
        /// Additional permissions required: the owner of the access token must have a specific role
        /// (`Director` or `Personnel_Manager`) in the corporation in order to access this data.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMembersLimit>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_corporation_membership`](crate::scope::CorporationsScopes::read_corporation_membership):
        ///   `esi-corporations.read_corporation_membership.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve the member limit for
        ///
        /// # Returns
        /// An ESI request builder that returns the maximum number of members the corporation can have when sent.
        auth fn get_corporation_member_limit(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<i32>
        method = Method::GET;
        url = "{}/corporations/{}/members/limit";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_corporation_membership())
            .build();
    }

    define_esi_endpoint! {
        /// Fetches a list of title IDs for each member of the provided corporation ID
        ///
        /// Returns information associating member character IDs with their title IDs within the corporation.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMembersTitles>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_titles`](crate::scope::CorporationsScopes::read_titles):
        ///   `esi-corporations.read_titles.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve member titles for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of member character IDs and their associated title IDs when sent.
        auth fn get_corporation_members_titles(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationMemberTitles>>
        method = Method::GET;
        url = "{}/corporations/{}/members/titles";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_titles())
            .build();
    }

    define_esi_endpoint! {
        /// Fetches a list of tracking information for each character part of the provided corporation ID
        ///
        /// Returns data such as member start dates, logon/logoff timestamps, location, and ship type.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMembersTracking>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::track_members`](crate::scope::CorporationsScopes::track_members):
        ///   `esi-corporations.track_members.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve member tracking for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of tracking information for each corporation member when sent.
        auth fn track_corporation_members(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationMemberTracking>>
        method = Method::GET;
        url = "{}/corporations/{}/membertracking";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().track_members())
            .build();
    }

    define_esi_endpoint! {
        /// Fetches a list of roles for each character part of the provided corporation ID
        ///
        /// Returns information about which roles each member has been granted within the corporation.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdRoles>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_corporation_membership`](crate::scope::CorporationsScopes::read_corporation_membership):
        ///   `esi-corporations.read_corporation_membership.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve member roles for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of roles for each corporation member when sent.
        auth fn get_corporation_member_roles(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationMemberRoles>>
        method = Method::GET;
        url = "{}/corporations/{}/roles";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_corporation_membership())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves a paginated list of up to a month of role history for the provided corporation ID
        ///
        /// Returns historical records of role changes for corporation members.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdRolesHistory>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_corporation_membership`](crate::scope::CorporationsScopes::read_corporation_membership):
        ///   `esi-corporations.read_corporation_membership.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve role history for
        /// - `page`            (`i32`): The page of role history to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of role change history for the corporation when sent.
        auth fn get_corporation_member_roles_history(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationMemberRolesHistory>>
        method = Method::GET;
        url = "{}/corporations/{}/roles/history";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_corporation_membership())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves a paginated list of shareholders for the provided corporation ID
        ///
        /// Returns information about who owns shares in the corporation.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdShareholders>
        ///
        /// # Required Scopes
        /// - [`WalletScopes::read_corporation_wallets`](crate::scope::WalletScopes::read_corporation_wallets):
        ///   `esi-wallet.read_corporation_wallets.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve shareholders for
        /// - `page`            (`i32`): The page of shareholders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of shareholders for the corporation when sent.
        auth fn get_corporation_shareholders(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationShareholder>>
        method = Method::GET;
        url = "{}/corporations/{}/shareholders";
        required_scopes = ScopeBuilder::new()
            .wallet(WalletScopes::new().read_corporation_wallets())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves a paginated list of NPC standing entries for the provided corporation ID
        ///
        /// Returns the corporation's standings with various NPC entities.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdStandings>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_standings`](crate::scope::CorporationsScopes::read_standings):
        ///   `esi-corporations.read_standings.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve standings for
        /// - `page`            (`i32`): The page of standings to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of NPC standings for the corporation when sent.
        auth fn get_corporation_standings(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<Standing>>
        method = Method::GET;
        url = "{}/corporations/{}/standings";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_standings())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves a paginated list of starbases (POSes) for the provided corporation ID
        ///
        /// Returns a list of starbase IDs and system IDs for starbases owned by the corporation.
        ///
        /// Additional permissions required: the owner of the access token must be a director in the
        /// corporation or have the `Station_Manager` role.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdStarbases>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_starbases`](crate::scope::CorporationsScopes::read_starbases):
        ///   `esi-corporations.read_starbases.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve starbases for
        /// - `page`            (`i32`): The page of starbases to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of starbases owned by the corporation when sent.
        auth fn get_corporation_starbases(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationStarbase>>
        method = Method::GET;
        url = "{}/corporations/{}/starbases";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_starbases())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves details for a starbase (POS) for the provided starbase ID & corporation ID
        ///
        /// Returns detailed information about a specific starbase including fuel levels and status.
        ///
        /// Additional permissions required: the owner of the access token must be a director in the
        /// corporation or have the `Station_Manager` role.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdStarbasesStarbaseId>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_starbases`](crate::scope::CorporationsScopes::read_starbases):
        ///   `esi-corporations.read_starbases.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation that owns the starbase
        /// - `starbase_id`     (`i64`): The ID of the starbase to retrieve details for
        /// - `system_id`       (`i64`): The ID of the solar system where the starbase is located
        ///
        /// # Returns
        /// An ESI request builder that returns detailed information about the specified starbase when sent.
        auth fn get_starbase_detail(
            access_token: &str,
            corporation_id: i64,
            starbase_id: i64;
            system_id: i64
        ) -> EsiRequest<CorporationStarbaseDetails>
        method = Method::GET;
        url = "{}/corporations/{}/starbases/{}";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_starbases())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves a paginated list of structure information for the provided corporation ID
        ///
        /// Returns information about Upwell structures (Citadels, Engineering Complexes, Refineries, etc.)
        /// owned by the corporation.
        ///
        /// Additional permissions required: the owner of the access token must be a director in the
        /// corporation or have the `Station_Manager` role.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdStructures>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_structures`](crate::scope::CorporationsScopes::read_structures):
        ///   `esi-corporations.read_structures.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve structures for
        /// - `page`            (`i32`): The page of structures to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of structures owned by the corporation when sent.
        auth fn get_corporation_structures(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationStructure>>
        method = Method::GET;
        url = "{}/corporations/{}/structures";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_structures())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves a list of corporation titles and their respective roles for the provided corporation ID
        ///
        /// Returns the defined titles within the corporation and what roles each title grants.
        ///
        /// Additional permissions required: the owner of the access token must hold the `director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdTitles>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_titles`](crate::scope::CorporationsScopes::read_titles):
        ///   `esi-corporations.read_titles.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve titles for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of corporation titles and their associated roles when sent.
        auth fn get_corporation_titles(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationTitle>>
        method = Method::GET;
        url = "{}/corporations/{}/titles";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_titles())
            .build();
    }
}
