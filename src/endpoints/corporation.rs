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
    Corporation, CorporationAllianceHistory, CorporationSecureContainerLog,
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
}
