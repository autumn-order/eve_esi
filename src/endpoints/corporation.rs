//! # EVE ESI Corporation Endpoints
//!
//! This module provides the [`CorporationApi`] struct and associated methods for accessing
//! corporation-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! # Methods
//! - [`CorporationApi::get_corporation_information`]: Fetches a corporation’s public information from ESI using the corporation ID

use crate::error::Error;
use crate::model::asset::Blueprint;
use crate::model::corporation::{
    Corporation, CorporationAllianceHistory, CorporationDivisions, CorporationFacilities,
    CorporationIcon, CorporationIssuedMedal, CorporationMedal, CorporationSecureContainerLog,
};
use crate::oauth2::scope::CorporationScopes;
use crate::{Client, ScopeBuilder};

/// Provides methods for accessing corporation-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct CorporationApi<'a> {
    client: &'a Client,
}

impl<'a> CorporationApi<'a> {
    /// Creates a new instance of `CorporationApi`.
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    ///
    /// # Returns
    /// - [`CorporationApi`]: Struct providing methods to interact with corporation ESI endpoints
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
        /// - [`CorporationScopes::read_blueprints`](crate::oauth2::scope::CorporationScopes::read_blueprints):
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
            corporation_id: i64,
            page: i32
        ) -> Result<Vec<Blueprint>, Error>
        url = "{}/corporations/{}/blueprints?page={}";
        label = "blueprints";
        required_scopes = ScopeBuilder::new().corporation(CorporationScopes::new().read_blueprints()).build();
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
        /// - [`CorporationScopes::read_container_logs`](crate::oauth2::scope::CorporationScopes::read_container_logs):
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
            corporation_id: i64,
            page: i32
        ) -> Result<Vec<CorporationSecureContainerLog>, Error>
        url = "{}/corporations/{}/containers/logs?page={}";
        label = "audit secure container log entries";
        required_scopes = ScopeBuilder::new().corporation(CorporationScopes::new().read_container_logs()).build();
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
        /// - [`CorporationScopes::read_divisions`](crate::oauth2::scope::CorporationScopes::read_divisions):
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
        required_scopes = ScopeBuilder::new().corporation(CorporationScopes::new().read_divisions()).build();
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
        /// - [`CorporationScopes::read_facilities`](crate::oauth2::scope::CorporationScopes::read_facilities):
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
        required_scopes = ScopeBuilder::new().corporation(CorporationScopes::new().read_facilities()).build();
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
        /// - [`CorporationScopes::read_medals`](crate::oauth2::scope::CorporationScopes::read_medals):
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
            corporation_id: i64,
            page: i32
        ) -> Result<Vec<CorporationMedal>, Error>
        url = "{}/corporations/{}/medals?page={}";
        label = "medals";
        required_scopes = ScopeBuilder::new().corporation(CorporationScopes::new().read_medals()).build();
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
        /// - [`CorporationScopes::read_medals`](crate::oauth2::scope::CorporationScopes::read_medals):
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
            corporation_id: i64,
            page: i32
        ) -> Result<Vec<CorporationIssuedMedal>, Error>
        url = "{}/corporations/{}/medals/issued?page={}";
        label = "medals";
        required_scopes = ScopeBuilder::new().corporation(CorporationScopes::new().read_medals()).build();
    }

    define_endpoint! {
        /// Fetches a list of character IDs of all members part of the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdMembers>
        ///
        /// # Arguments
        /// - `corporation_id` ([`i64`]): The ID of the corporation to retrieve members for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: List of character IDs of all members part of the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_corporation_members(
            corporation_id: i64
        ) -> Result<Vec<i64>, Error>
        url = "{}/corporations/{}/members";
        label = "character IDs of all members";
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
        /// - [`CorporationScopes::track_members`](crate::oauth2::scope::CorporationScopes::track_members):
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
        required_scopes = ScopeBuilder::new().corporation(CorporationScopes::new().track_members()).build();
    }
}
