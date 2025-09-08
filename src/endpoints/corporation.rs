//! Corporation Endpoints for EVE Online's ESI API.
//!
//! This module provides the [`CorporationApi`] struct and associated methods for accessing
//! corporation-related endpoints of the EVE Online ESI (EVE Swagger Interface) API.
//!
//! The [`CorporationApi`] acts as a high-level interface for retrieving public information
//! and affiliations for EVE Online corporations. It requires an [`Client`] instance
//! to perform HTTP requests to the ESI endpoints.
//!
//! # Features
//! - Fetch public information about a corporation by corporation ID
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
//!     // Get information about the corporation The Order of Autumn (id: 98785281)
//!     let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();
//!     println!("Corporation name: {}", corporation.name);
//! }
//! ```

use crate::error::Error;
use crate::model::corporation::Corporation;
use crate::Client;

/// Provides methods for accessing corporation-related endpoints of the EVE Online ESI API.
///
/// The `CorporationApi` struct acts as an interface for retrieving information about EVE Online corporations
/// using the ESI API. It requires an [`Client`] for making HTTP requests to the ESI endpoints.
///
/// See the [module-level documentation](self) for an overview and usage example.
pub struct CorporationApi<'a> {
    client: &'a Client,
}

impl<'a> CorporationApi<'a> {
    /// Creates a new instance of `CorporationApi`.
    ///
    /// # Arguments
    /// - `client` - The [`Client`] used for making HTTP requests to the ESI endpoints.
    ///
    /// # Returns
    /// Returns a new instance of `CorporationApi`.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves information about a corporation from EVE Online's ESI API using the provided corporation ID.
    ///
    /// This endpoint fetches corporation information, returning data such as name, ticker, member count
    /// and other relevant information.
    ///
    /// # Arguments
    /// - `corporation_id` - The ID of the corporation to retrieve information for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Corporation`] - The corporation information if the request was successful.
    /// - [`EsiError`] - An error if the request failed (e.g. corporation not found, network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationId)
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
    ///     // Get information about the corporation The Order of Autumn (id: 98785281)
    ///     let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();
    ///     println!("Corporation name: {}", corporation.name);
    /// }
    /// ```
    pub async fn get_corporation_information(
        &self,
        corporation_id: i32,
    ) -> Result<Corporation, Error> {
        let url = format!(
            "{}/corporations/{}/",
            self.client.inner.esi_url, corporation_id
        );

        Ok(self.client.get_from_public_esi::<Corporation>(&url).await?)
    }
}
