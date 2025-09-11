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

use std::time::Instant;

use log::{debug, error, info};

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
    pub async fn get_corporation_information(
        &self,
        corporation_id: i64,
    ) -> Result<Corporation, Error> {
        let url = format!(
            "{}/corporations/{}/",
            self.client.inner.esi_url, corporation_id
        );

        let message = format!(
            "Fetching corporation information for corporation ID {} from \"{}\"",
            corporation_id, url
        );

        debug!("{}", message);

        let start_time = Instant::now();

        // Fetch corporation information from ESI
        let result = self
            .client
            .esi()
            .get_from_public_esi::<Corporation>(&url)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(corporation) => {
                let message = format!(
                    "Successfully fetched corporation information for corporation ID: {} (took {}ms)",
                    corporation_id,
                    elapsed.as_millis()
                );

                info!("{}", message);

                Ok(corporation)
            }
            Err(err) => {
                let message = format!(
                    "Successfully fetched corporation information for corporation ID: {} (took {}ms)",
                    corporation_id,
                    elapsed.as_millis()
                );

                error!("{}", message);

                Err(err.into())
            }
        }
    }
}
