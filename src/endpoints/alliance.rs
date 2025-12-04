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
    model::{
        alliance::{Alliance, AllianceIcons},
        esi::EsiRequest,
    },
    Client, Error,
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
        /// - `Ok(request)`: Request builder for a vector of alliance IDs
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        pub fn list_all_alliances() -> Result<EsiRequest<Vec<i64>>, Error>
        method = Method::GET;
        url = "{}/alliances";
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
        /// - `Ok(request)`: Request builder for alliance public information
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        pub fn get_alliance_information(
            alliance_id: i64
        ) -> Result<EsiRequest<Alliance>, Error>
        method = Method::GET;
        url = "{}/alliances/{}";
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
        /// - `Ok(request)`: Request builder for a vector of corporation IDs
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        pub fn list_alliance_corporations(
            alliance_id: i64
        ) -> Result<EsiRequest<Vec<i64>>, Error>
        method = Method::GET;
        url = "{}/alliances/{}/corporations";
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
        /// - `Ok(request)`: Request builder for alliance icon URLs
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        pub fn get_alliance_icon(
            alliance_id: i64
        ) -> Result<EsiRequest<AllianceIcons>, Error>
        method = Method::GET;
        url = "{}/alliances/{}/icons";
    }
}
