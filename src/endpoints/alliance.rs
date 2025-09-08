//! Alliance Endpoints for EVE Online's ESI API.
//!
//! This module provides the [`AllianceApi`] struct and associated methods for accessing
//! alliance-related endpoints of the EVE Online ESI (EVE Swagger Interface) API.
//!
//! The [`AllianceApi`] acts as a high-level interface for retrieving public information
//! and affiliations for EVE Online alliances. It requires an [`Client`] instance
//! to perform HTTP requests to the ESI endpoints.
//!
//! # Features
//! - Fetch public information about an alliance by alliance ID
//!
//! # References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//!
//! # Usage Example
//! ```no_run
//! #[tokio::main]
//! async fn main() {
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

use crate::{model::alliance::Alliance, Client, Error};

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

    /// Retrieves information about a specific alliance from EVE Online's ESI API.
    ///
    /// This endpoint fetches public data about an alliance including name, ticker, date founded,
    /// executor corporation, and faction.
    ///
    /// # Arguments
    /// - `alliance_id` - The unique identifier for the alliance to look up
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Alliance`] - The alliance data if successfully retrieved
    /// - [`EsiError`] - An error if the request failed (e.g., alliance not found, network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceId).
    ///
    /// # Example
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let esi_client = eve_esi::Client::builder()
    ///         .user_agent("MyApp/1.0 (contact@example.com)")
    ///         .build()
    ///         .expect("Failed to build Client");
    ///
    ///     // Get information about The Autumn alliance (id: 99013534)
    ///     let alliance = esi_client.alliance().get_alliance_information(99013534).await.unwrap();
    ///     println!("Alliance name: {}", alliance.name);
    /// }
    /// ```
    pub async fn get_alliance_information(&self, alliance_id: i32) -> Result<Alliance, Error> {
        let url = format!("{}/alliances/{}/", self.client.esi_url, alliance_id);

        Ok(self.client.get_from_public_esi::<Alliance>(&url).await?)
    }
}
