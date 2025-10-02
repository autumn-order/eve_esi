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
//! | [`CorporationEndpoints::get_corporation_information`] | Fetches a corporation’s public information from ESI using the corporation ID |
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

use crate::error::Error;
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

/// Provides methods for accessing corporation-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct CorporationEndpoints<'a> {
    client: &'a Client,
}

impl<'a> CorporationEndpoints<'a> {
    /// Creates a new instance of [`CorporationEndpoints`].
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)e
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Fetches a list of all NPC corporation IDs in EVE Online
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsNpccorps>
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: List of IDs of all NPC corporations in EVE Online
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_npc_corporations(
        ) -> Result<Vec<i64>, Error>
        url = "{}/corporations/npccorps";
        label = "NPC corporations";
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - [`Corporation`]: The corporation information if the request was successful.
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_corporation_information(
            corporation_id: i64
        ) -> Result<Corporation, Error>
        url = "{}/corporations/{}";
        label = "public information";
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationAllianceHistory`]`>`: List of entries for the corporation's alliance
        ///   history.
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_alliance_history(
            corporation_id: i64
        ) -> Result<Vec<CorporationAllianceHistory>, Error>
        url = "{}/corporations/{}/alliancehistory";
        label = "alliance history";
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`Blueprint`]`>`: List of blueprint entries for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_blueprints(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<Blueprint>, Error>
        url = "{}/corporations/{}/blueprints";
        label = "blueprints";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_blueprints()).build();
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationSecureContainerLog`]`>`: List of ALSC log entries for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_all_corporation_alsc_logs(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<CorporationSecureContainerLog>, Error>
        url = "{}/corporations/{}/containers/logs";
        label = "audit secure container log entries";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_container_logs()).build();
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - [`CorporationDivisions`]: Struct containing entries for corporation hangar & wallet divisions
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_divisions(
            access_token: &str,
            corporation_id: i64
        ) -> Result<CorporationDivisions, Error>
        url = "{}/corporations/{}/divisions";
        label = "hangar & wallet divisions";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_divisions()).build();
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationFacilities`]`>`: List of corporation industry facilities
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_facilities(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<CorporationFacilities>, Error>
        url = "{}/corporations/{}/facilities";
        label = "industry facilities";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_facilities()).build();
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - [`CorporationIcon`]: The corporation icon URLs
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_corporation_icon(
            corporation_id: i64
        ) -> Result<CorporationIcon, Error>
        url = "{}/corporations/{}/icons";
        label = "icons";
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationMedal`]`>`: List of corporation medal entries
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_medals(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<CorporationMedal>, Error>
        url = "{}/corporations/{}/medals";
        label = "medals";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_medals()).build();
    }

    define_endpoint! {
        /// Fetches a paginated list of issued medals for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
        /// the corporation to access this information.
        ///
        /// This endpoint differs from [`Self::get_corporation_medals`] in that it represents who issued the medal
        /// and who the medal was issued to while [`Self::get_corporation_medals`] describes the medal itself.
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
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve medals for
        /// - `page`            (`i32`): The page of medals to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationIssuedMedal`]`>`: List of issued corporation medal entries
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_issued_medals(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<CorporationIssuedMedal>, Error>
        url = "{}/corporations/{}/medals/issued";
        label = "medals";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_medals()).build();
    }

    define_endpoint! {
        /// Fetches a list of character IDs of all members part of the provided corporation ID
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
        /// - `corporation_id` ([`i64`]): The ID of the corporation to retrieve members for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: List of character IDs of all members part of the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_members(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<i64>, Error>
        url = "{}/corporations/{}/members";
        label = "character IDs of all members";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_corporation_membership()).build();
    }

    define_endpoint! {
        /// Fetches the member limit of the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMembersLimit>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::track_members`](crate::scope::CorporationsScopes::track_members):
        ///   `esi-corporations.track_members.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve member limit for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `i64`: Integer representing the member limit of the corporation not including the CEO
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_member_limit(
            access_token: &str,
            corporation_id: i64
        ) -> Result<i64, Error>
        url = "{}/corporations/{}/members/limit";
        label = "member limit";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().track_members()).build();
    }

    define_endpoint! {
        /// Fetches a list of title IDs for each member of the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationMemberTitles`]`>`: List of title IDs for each member of the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_members_titles(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<CorporationMemberTitles>, Error>
        url = "{}/corporations/{}/members/titles";
        label = "member titles";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_titles()).build();
    }

    define_endpoint! {
        /// Fetches a list of tracking information for each character part of the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
        /// the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMembertracking>
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationMemberTracking`]`>`: List of tracking information for each character part of the provided
        ///   corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get track_corporation_members(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<CorporationMemberTracking>, Error>
        url = "{}/corporations/{}/membertracking";
        label = "member tracking";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().track_members()).build();
    }

    define_endpoint! {
        /// Fetches a list of roles for each character part of the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Personnel Manager` role within
        /// the corporation or any other grantable role to access this information.
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
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve roles for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationMemberRoles`]`>`: List of roles for each character part of the provided
        ///   corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_member_roles(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<CorporationMemberRoles>, Error>
        url = "{}/corporations/{}/roles";
        label = "member roles";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_corporation_membership()).build();
    }

    define_endpoint! {
        /// Retrieves a paginated list of up to a month of role history for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
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
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve member roles history for
        /// - `page`            (`i32`): The page of roles history to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationMemberRolesHistory`]`>`: Paginated list of role history for each character
        ///   part of the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_member_roles_history(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<CorporationMemberRolesHistory>, Error>
        url = "{}/corporations/{}/roles/history";
        label = "member roles";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_corporation_membership()).build();
    }

    define_endpoint! {
        /// Retrieves a paginated list of shareholders for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationShareholder`]`>`: Paginated list of shareholders for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_shareholders(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<CorporationShareholder>, Error>
        url = "{}/corporations/{}/shareholders";
        label = "shareholders";
        required_scopes = ScopeBuilder::new().wallet(WalletScopes::new().read_corporation_wallets()).build();
    }

    define_endpoint! {
        /// Retrieves a paginated list of NPC standing entries for the provided corporation ID
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
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve NPC standings for
        /// - `page`            (`i32`): The page of corporation NPC standings to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`Standing`]`>`: Paginated list of NPC standing entries for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_standings(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<Standing>, Error>
        url = "{}/corporations/{}/standings";
        label = "NPC standings";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_standings()).build();
    }

    define_endpoint! {
        /// Retrieves a paginated list of starbases (POSes) for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
        /// the corporation to access this information.
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
        /// - `corporation_id`  (`i64`): The ID of the corporation to starbases (POSes) for
        /// - `page`            (`i32`): The page of corporation NPC standings to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationStarbase`]`>`: Paginated list of starbases (POSes) for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_starbases(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<CorporationStarbase>, Error>
        url = "{}/corporations/{}/starbases";
        label = "starbases (POSes)";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_starbases()).build();
    }

    define_endpoint! {
        /// Retrieves details for a starbase (POS) for the provided starbase ID & corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
        /// the corporation to access this information.
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
        /// - `corporation_id`  (`i64`): The ID of the corporation to starbases (POSes) for
        /// - `starbase_id`     (`i64`): The unique ID of the corporation owned starbase (POS) to retrieve
        /// - `system_id`       (`i64`): The unique ID of the system the starbase (POS) is located in
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`CorporationStarbaseDetails`]: Details of the starbase for the provided starbase_id & corporation_id
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_starbase_detail(
            access_token: &str,
            corporation_id: i64,
            starbase_id: i64;
            system_id: i64
        ) -> Result<CorporationStarbaseDetails, Error>
        url = "{}/corporations/{}/starbases/{}";
        label = "a starbase's (POS) details";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_starbases()).build();
    }

    define_endpoint! {
        /// Retrieves a paginated list of structure information for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Station Manager` role within
        /// the corporation to access this information.
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
        /// - `corporation_id`  (`i64`): The ID of the corporation retrieve structures information for
        /// - `page`            (`i32`): The page of structures information to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`CorporationStructure`]: Paginated list of structure information for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_structures(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<Vec<CorporationStructure>, Error>
        url = "{}/corporations/{}/structures";
        label = "structures";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_structures()).build();
    }

    define_endpoint! {
        /// Retrieves a list of corporation titles and their respective roles for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Director` role within
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
        /// - `corporation_id`  (`i64`): The ID of the corporation retrieve titles for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationTitle`]`>`: List of corporation titles and their respective roles
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_titles(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<CorporationTitle>, Error>
        url = "{}/corporations/{}/titles";
        label = "titles";
        required_scopes = ScopeBuilder::new().corporations(CorporationsScopes::new().read_titles()).build();
    }
}
