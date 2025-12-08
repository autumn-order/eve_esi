//! # EVE ESI Alliance Endpoints
//!
//! This module provides the [`AllianceEndpoints`] struct and associated methods for accessing
//! alliance-related ESI endpoints. All endpoints in this module are public and do not require
//! authentication.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>

use crate::{
    esi::EsiRequest,
    model::alliance::{Alliance, AllianceIcons},
    Client,
};
use reqwest::Method;

/// Provides methods for accessing alliance-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct AllianceEndpoints<'a> {
    client: &'a Client,
}

impl<'a> AllianceEndpoints<'a> {
    /// Creates a new instance of [`AllianceEndpoints].
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # Arguments
    /// - `client`: ESI client used for making HTTP requests to the ESI endpoints
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_esi_endpoint! {
        /// Retrieves a list of IDs of every alliance in EVE Online.
        ///
        /// The response contains a vector of alliance IDs representing all alliances
        /// in the game.
        ///
        /// # ESI Documentation
        /// <https://developers.eveonline.com/api-explorer#/operations/GetAlliances>
        ///
        /// # Returns
        /// An ESI request builder that returns a vector of alliance IDs when sent.
        pub fn list_all_alliances() -> EsiRequest<Vec<i64>>
        method = Method::GET;
        path = "/alliances";
    }

    define_esi_endpoint! {
        /// Fetches an alliance's public information from ESI.
        ///
        /// The response contains details including the alliance name, ticker, founding date,
        /// creator information, and executor corporation. This data is publicly available
        /// for all alliances in EVE Online.
        ///
        /// # ESI Documentation
        /// <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceId>
        ///
        /// # Arguments
        /// - `alliance_id`: The ID of the alliance to retrieve information for
        ///
        /// # Returns
        /// An ESI request builder that returns alliance public information when sent.
        pub fn get_alliance_information(
            alliance_id: i64
        ) -> EsiRequest<Alliance>
        method = Method::GET;
        path = "/alliances/{}";
    }

    define_esi_endpoint! {
        /// Retrieves the IDs of all corporations in an alliance.
        ///
        /// The response contains a vector of corporation IDs that are current members
        /// of the specified alliance. This list updates as corporations join or leave
        /// the alliance.
        ///
        /// # ESI Documentation
        /// <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceIdCorporations>
        ///
        /// # Arguments
        /// - `alliance_id`: ID of the alliance to fetch corporation IDs for
        ///
        /// # Returns
        /// An ESI request builder that returns a vector of corporation IDs belonging to the alliance when sent.
        pub fn list_alliance_corporations(
            alliance_id: i64
        ) -> EsiRequest<Vec<i64>>
        method = Method::GET;
        path = "/alliances/{}/corporations";
    }

    define_esi_endpoint! {
        /// Retrieves the 128x128 and 64x64 icon URLs for an alliance.
        ///
        /// The response contains URLs pointing to the alliance's logo images hosted on
        /// EVE Online's image server. Both icon sizes are provided and can be used
        /// directly in applications or websites.
        ///
        /// # ESI Documentation
        /// <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceIdIcons>
        ///
        /// # Arguments
        /// - `alliance_id`: ID of the alliance to fetch icons for
        ///
        /// # Returns
        /// An ESI request builder that returns the alliance's 128x128 and 64x64 icon URLs when sent.
        pub fn get_alliance_icon(
            alliance_id: i64
        ) -> EsiRequest<AllianceIcons>
        method = Method::GET;
        path = "/alliances/{}/icons";
    }
}
