//! # EVE ESI Alliance Endpoints
//!
//! This module provides the [`AllianceApi`] struct and associated methods for accessing
//! alliance-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (4)
//! ### Public (4)
//! - [`AllianceApi::list_all_alliances`]: Retrieves a list of IDs of every alliance in EVE Online
//! - [`AllianceApi::get_alliance_information`]: Retrieves public information for the requested alliance_id
//! - [`AllianceApi::list_alliance_corporations`]: Retrieves the IDs of all corporations part of the requested alliance_id
//! - [`AllianceApi::get_alliance_icon`]: Get the 128x128 & 64x64 icon URLs for the requested alliance_id

use crate::{
    model::alliance::{Alliance, AllianceIcons},
    Client, Error,
};

/// Provides methods for accessing character-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct AllianceEndpoints<'a> {
    client: &'a Client,
}

impl<'a> AllianceEndpoints<'a> {
    /// Creates a new instance of `AllianceApi`.
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    ///
    /// # Returns
    /// - [`AllianceApi`]: Struct providing methods to interact with alliance ESI endpoints
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Retrieves a list of IDs of every alliance in EVE Online
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetAlliances>
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`i64`]>: A vec of every alliance ID in EVE Online
        /// - [`Error`]: An error if the fetch request failed
        pub_get list_all_alliances() -> Result<Vec<i64>, Error>
        url = "{}/alliances";
        label = "list of all alliance IDs";
    }

    define_endpoint! {
        /// Fetches an alliance's public information from ESI using the alliance ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        ///- <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceId>
        ///
        /// # Arguments
        /// - `alliance_id` ([`i64`]): The ID of the alliance to retrieve information for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`Alliance`]: The alliance data if successfully retrieved
        /// - [`Error`]: An error if the fetch request failed
        pub_get get_alliance_information(
            alliance_id: i64
        ) -> Result<Alliance, Error>
        url = "{}/alliances/{}";
        label = "public information";
    }

    define_endpoint! {
        /// Retrieves the IDs of all corporations part of the provided alliance_id
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceIdCorporations>
        ///
        /// # Arguments
        /// - `alliance_id` ([`i64`]): ID of the alliance to fetch corporation IDs for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`i64`]>: A vec of the ID of every corporation part of the alliance
        /// - [`Error`]: An error if the fetch request failed
        pub_get list_alliance_corporations(
            alliance_id: i64
        ) -> Result<Vec<i64>, Error>
        url = "{}/alliances/{}/corporations";
        label = "alliance corporation IDs";
    }

    define_endpoint! {
        /// Get the 128x128 & 64x64 icon URLs for an alliance
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceIdIcons>
        ///
        /// # Arguments
        /// - `alliance_id` ([`i64`]): ID of the alliance to fetch icons for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`AllianceIcons`]: A struct with URLs for the 128x128 & 64x64 icons for an alliance
        /// - [`Error`]: An error if the fetch request failed
        pub_get get_alliance_icon(
            alliance_id: i64
        ) -> Result<AllianceIcons, Error>
        url = "{}/alliances/{}/icons";
        label = "alliance icons";
    }
}
