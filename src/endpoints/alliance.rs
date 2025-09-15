//! # EVE ESI Alliance Endpoints
//!
//! This module provides the [`AllianceApi`] struct and associated methods for accessing
//! alliance-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! # Methods
//! - [`AllianceApi::list_all_alliances`]: Retrieves a list of IDs of every alliance in EVE Online
//! - [`AllianceApi::get_alliance_information`]: Retrieves public information for the requested alliance_id
//! - [`AllianceApi::list_alliance_corporations`]: Retrieves the IDs of all corporations part of the requested alliance_id
//! - [`AllianceApi::get_alliance_icon`]: Get the 128x128 & 64x64 icon URLs for the requested alliance_id

use std::time::Instant;

use crate::{
    model::alliance::{Alliance, AllianceIcons},
    Client, Error,
};

/// Provides methods for accessing character-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct AllianceApi<'a> {
    client: &'a Client,
}

impl<'a> AllianceApi<'a> {
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
    pub async fn list_all_alliances(&self) -> Result<Vec<i64>, Error> {
        let url = format!("{}/alliances", self.client.inner.esi_url);

        debug!("Fetching list of all alliance IDs from \"{}\"", url);

        let start_time = Instant::now();

        // Fetch all alliances
        let result = self
            .client
            .esi()
            .get_from_public_esi::<Vec<i64>>(&url)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(alliances) => {
                info!(
                    "Successfully fetched IDs for {} alliances (took {}ms)",
                    alliances.len(),
                    elapsed.as_millis()
                );

                Ok(alliances)
            }
            Err(err) => {
                error!(
                    "Failed to fetch list of all alliance IDs after {}ms due to error: {:#?}",
                    elapsed.as_millis(),
                    err
                );

                Err(err.into())
            }
        }
    }

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
    pub async fn get_alliance_information(&self, alliance_id: i64) -> Result<Alliance, Error> {
        let url = format!("{}/alliances/{}/", self.client.inner.esi_url, alliance_id);

        debug!(
            "Fetching alliance information for alliance ID {} from \"{}\"",
            alliance_id, url
        );

        let start_time = Instant::now();

        // Fetch alliance information from ESI
        let result = self
            .client
            .esi()
            .get_from_public_esi::<Alliance>(&url)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(alliance) => {
                info!(
                    "Successfully fetched alliance information for alliance ID: {} (took {}ms)",
                    alliance_id,
                    elapsed.as_millis()
                );

                Ok(alliance)
            }
            Err(err) => {
                error!(
                "Failed to fetch alliance information for alliance ID {} after {}ms due to error: {:#?}",
                alliance_id,
                elapsed.as_millis(),
                err);

                Err(err.into())
            }
        }
    }

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
    pub async fn list_alliance_corporations(&self, alliance_id: i64) -> Result<Vec<i64>, Error> {
        let url = format!(
            "{}/alliances/{}/corporations",
            self.client.inner.esi_url, alliance_id
        );

        debug!(
            "Fetching IDs of all corporations part of alliance ID {} from \"{}\"",
            alliance_id, url
        );

        let start_time = Instant::now();

        // Fetch all alliances
        let result = self
            .client
            .esi()
            .get_from_public_esi::<Vec<i64>>(&url)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(corporations) => {
                info!(     "Successfully fetched IDs for {} corporation(s) part of alliance ID {} (took {}ms)",
                corporations.len(),
                alliance_id,
                elapsed.as_millis());

                Ok(corporations)
            }
            Err(err) => {
                error!("Failed to fetch IDs of all corporations part of alliance ID {} after {}ms due to error: {:#?}",
                alliance_id,
                elapsed.as_millis(),
                err);

                Err(err.into())
            }
        }
    }

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
    pub async fn get_alliance_icon(&self, alliance_id: i64) -> Result<AllianceIcons, Error> {
        let url = format!(
            "{}/alliances/{}/icons",
            self.client.inner.esi_url, alliance_id
        );

        debug!(
            "Fetching icons URLs for alliance ID {} from \"{}\"",
            alliance_id, url
        );

        let start_time = Instant::now();

        // Fetch all alliances
        let result = self
            .client
            .esi()
            .get_from_public_esi::<AllianceIcons>(&url)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(icons) => {
                info!(
                    "Successfully fetched icon URLs for alliance ID {} (took {}ms)",
                    alliance_id,
                    elapsed.as_millis()
                );

                Ok(icons)
            }
            Err(err) => {
                error!(
                    "Failed to fetch icon URLs for alliance ID {} after {}ms due to error: {:#?}",
                    alliance_id,
                    elapsed.as_millis(),
                    err
                );

                Err(err.into())
            }
        }
    }
}
