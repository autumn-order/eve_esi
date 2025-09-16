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
use crate::model::corporation::Corporation;
use crate::Client;

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
        /// Fetches a corporation's public information from ESI using the corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationId>
        ///
        /// # Arguments
        /// - `corporation_id` ([`i32`]): The ID of the corporation to retrieve information for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`Corporation`] - The corporation information if the request was successful.
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_corporation_information(
            corporation_id: i64
        ) -> Result<Corporation, Error>
        url = "{}/corporations/{}/";
        label = "public information";
    }
}
