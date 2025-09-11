//! Alliance Endpoints for EVE Online's ESI API.
//!
//! This module provides the [`AllianceApi`] struct and associated methods for accessing
//! alliance-related endpoints of the EVE Online ESI (EVE Stable Infrastructure) API.
//!
//! # Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! # Methods
//! - [`AllianceApi::list_all_alliances`]: Retrieves a list of IDs of every alliance in EVE Online
//! - [`AllianceApi::get_alliance_information`]: Retrieves public information for the given alliance_id
//! - [`AllianceApi::list_alliance_corporations]: Retrieves the IDs of all corporations part of the provided alliance_id
//! - [`AllianceApi::get_alliance_icon]: Get the 128x128 & 64x64 icon URLs for an alliance
//!
//! # Usage Example
//! ```no_run
//! #[tokio::main]
//! async fn main() {
//!     // Build an ESI client
//!     let esi_client = eve_esi::Client::builder()
//!         .user_agent("MyApp/1.0 (contact@example.com)")
//!         .build()
//!         .expect("Failed to build Client");
//!
//!     // Get information about The Autumn alliance (id: 99013534)
//!     let alliance = esi_client.alliance().get_alliance_information(99013534).await.unwrap();
//!     println!("Alliance name: {}", alliance.name);
//! }
//! ```

use std::time::Instant;

use crate::{
    model::alliance::{Alliance, AllianceIcons},
    Client, Error,
};

use log::{debug, error, info};

/// Provides methods for accessing character-related endpoints of the EVE Online ESI API.
///
/// The `AllianceApi` struct acts as an interface for retrieving information about EVE Online alliances
/// using the ESI API. It requires an [`Client`] for making HTTP requests to the ESI endpoints.
///
/// See the [module-level documentation](self) for an overview and usage example.
pub struct AllianceApi<'a> {
    client: &'a Client,
}

impl<'a> AllianceApi<'a> {
    /// Creates a new instance of `AllianceApi`.
    ///
    /// # Arguments
    /// - `client` - The [`Client`] used for making HTTP requests to the ESI endpoints.
    ///
    /// # Returns
    /// Returns a new instance of `AllianceApi`.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves a list of IDs of every alliance in EVE Online
    ///
    /// # EVE ESI Reference
    /// - <https://developers.eveonline.com/api-explorer#/operations/GetAlliances>
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - `Vec<`[`i64`]`>`: A vec of every alliance ID in EVE Online
    /// - [`Error`]: An error if the fetch request failed
    pub async fn list_all_alliances(&self) -> Result<Vec<i64>, Error> {
        let url = format!("{}/alliances", self.client.inner.esi_url);

        let message = format!("Fetching list of all alliance IDs from {}", url);

        debug!("{}", message);

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
                let message = format!(
                    "Successfully fetched IDs for {} alliances (took {}ms)",
                    alliances.len(),
                    elapsed.as_millis()
                );

                info!("{}", message);

                Ok(alliances)
            }
            Err(err) => {
                let message = format!(
                    "Failed to fetch list of all alliance IDs after {}ms due to error: {:#?}",
                    elapsed.as_millis(),
                    err
                );

                error!("{}", message);

                Err(err.into())
            }
        }
    }

    /// Retrieves public information for the given alliance_id
    ///
    /// # EVE ESI Reference
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

        let message = format!(
            "Fetching alliance information for alliance ID {} from {}",
            alliance_id, url
        );

        debug!("{}", message);

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
                let message = format!(
                    "Successfully fetched alliance information for alliance ID: {} (took {}ms)",
                    alliance_id,
                    elapsed.as_millis()
                );

                info!("{}", message);

                Ok(alliance)
            }
            Err(err) => {
                let message = format!(
                    "Failed to fetch alliance information for alliance ID {} after {}ms due to error: {:#?}",
                    alliance_id,
                    elapsed.as_millis(),
                    err
                );

                error!("{}", message);

                Err(err.into())
            }
        }
    }

    /// Retrieves the IDs of all corporations part of the provided alliance_id
    ///
    /// # EVE ESI Reference
    /// - <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceIdCorporations>
    ///
    /// # Arguments
    /// - `alliance_id` ([`i64`]): ID of the alliance to fetch corporation IDs for
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - `Vec<`[`i64`]`>`: A vec of the ID of every corporation part of the alliance
    /// - [`Error`]: An error if the fetch request failed
    pub async fn list_alliance_corporations(&self, alliance_id: i64) -> Result<Vec<i64>, Error> {
        let url = format!(
            "{}/alliances/{}/corporations",
            self.client.inner.esi_url, alliance_id
        );

        let message = format!(
            "Fetching IDs of all corporations part of alliance ID {} from {}",
            alliance_id, url
        );

        debug!("{}", message);

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
                let message = format!(
                    "Successfully fetched IDs for {} corporation(s) part of alliance ID {} (took {}ms)",
                    corporations.len(),
                    alliance_id,
                    elapsed.as_millis()
                );

                info!("{}", message);

                Ok(corporations)
            }
            Err(err) => {
                let message = format!(
                    "Failed to fetch IDs of all corporations part of alliance ID {} after {}ms due to error: {:#?}",
                    alliance_id,
                    elapsed.as_millis(),
                    err
                );

                error!("{}", message);

                Err(err.into())
            }
        }
    }

    /// Get the 128x128 & 64x64 icon URLs for an alliance
    ///
    /// # EVE ESI Reference
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

        let message = format!(
            "Fetching icons URLs for alliance ID {} from {}",
            alliance_id, url
        );

        debug!("{}", message);

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
                let message = format!(
                    "Successfully fetched icon URLs for alliance ID {} (took {}ms)",
                    alliance_id,
                    elapsed.as_millis()
                );

                info!("{}", message);

                Ok(icons)
            }
            Err(err) => {
                let message = format!(
                    "Failed to fetch icon URLs for alliance ID {} after {}ms due to error: {:#?}",
                    alliance_id,
                    elapsed.as_millis(),
                    err
                );

                error!("{}", message);

                Err(err.into())
            }
        }
    }
}
