//! # EVE ESI Request Methods
//!
//! Provides utility methods for making requests to EVE Online's ESI. These
//! methods are used internally by the [`crate::endpoints`] module to make requests.
//!
//! Despite the use case intended primarily to be internal, these functions are exported publicly
//! to allow for using the ESI client to make requests to custom ESI routes. This is useful
//! for when this crate hasn't implemented an ESI route yet but you still wish to use the client
//! to make requests to the route.
//!
//! For usage regarding making ESI requests with the eve_esi crate, see the
//! [endpoints module documentation](crate::endpoints)
//!
//! ## Modules
//! - [`public`]: Methods for making public requests to ESI endpoints
//! - [`authenticated`]: Methods for making authenticated requests to ESI endpoints using an access token
//!
//! ## Usage
//!
//! ```no_run
//! use serde::{Serialize, Deserialize};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup a basic Client with a user agent to identify requests
//!     let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)";
//!     let esi_client = eve_esi::Client::new(user_agent).expect("Failed to build ESI Client");
//!
//!     // Define the struct to deserialize the ESI response to
//!     #[derive(Serialize, Deserialize)]
//!     pub struct CharacterAffiliations {
//!         pub alliance_id: Option<i64>,
//!         pub character_id: i64,
//!         pub corporation_id: i64,
//!         pub faction_id: Option<i64>,
//!     };
//!
//!     // Define the URL to make the request to
//!     let esi_endpoint_url = "https://esi.evetech.net/characters/affiliation/";
//!
//!     // Make the request with the earlier defined struct
//!     // - The first type, `<Vec<CharacterAffiliations>`, represents the response body to deserialize
//!     // - The second type, `Vec<i64>`, represents the request body to serialize (not applicable to GET requests)
//!     let character_ids = vec![2114794365];
//!
//!     let character_affiliations = esi_client
//!         .esi()
//!         .post_to_public_esi::<Vec<CharacterAffiliations>, Vec<i64>>(&esi_endpoint_url, &character_ids)
//!         .await;
//! }
//! ```

pub mod authenticated;
pub mod public;

mod util;

use serde::de::DeserializeOwned;

use crate::{model::esi::EsiRequest, Client, Error};

/// Provides utility methods for making requests EVE Online's ESI endpoints
///
/// See the [module-level documentation](super) for an overview, methods, & usage example.
pub struct EsiApi<'a> {
    pub(crate) client: &'a Client,
}

impl Client {
    /// Access to utility functions to make ESI requests
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
    pub fn esi(&self) -> self::EsiApi<'_> {
        self::EsiApi::new(self)
    }
}

impl<'a> EsiApi<'a> {
    /// Creates a new instance of [`EsiApi`]
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Make a request to ESI using the provided [`EsiRequest`] configuration.
    ///
    /// This method consolidates all ESI request logic, handling both authenticated and public requests
    /// based on the configuration in the [`EsiRequest`] struct. It automatically:
    /// - Validates access tokens if present (expiration & scope checks)
    /// - Adds authentication headers for authenticated requests
    /// - Applies all custom headers from the request
    /// - Handles request body for POST, PUT, and PATCH methods
    /// - Returns deserialized response data
    ///
    /// # Arguments
    /// - `request`: The configured [`EsiRequest`] containing endpoint, method, headers, and authentication details
    ///
    /// # Returns
    /// A Result containing the deserialized response data or an error
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`
    ///
    /// # Example
    /// ```no_run
    /// use eve_esi::{EsiRequest, Client};
    /// use reqwest::Method;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct ServerStatus {
    ///     players: i32,
    ///     server_version: String,
    ///     start_time: String,
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)";
    /// let client = Client::new(user_agent)?;
    ///
    /// let request = EsiRequest::new("https://esi.evetech.net/latest/status/")
    ///     .with_method(Method::GET)
    ///     .with_compatibility_date("2025-11-06");
    ///
    /// let status: ServerStatus = client.esi().request(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn request<T: DeserializeOwned>(&self, request: EsiRequest) -> Result<T, Error> {
        // Validate token if this is an authenticated request
        if let Some(access_token) = request.access_token() {
            self.validate_token_before_request(access_token, request.required_scopes().clone())
                .await?;
        }

        let reqwest_client = &self.client.inner.reqwest_client;

        // Build the request with the appropriate HTTP method
        let mut req_builder = reqwest_client.request(request.method().clone(), request.endpoint());

        // Add authorization header if access token is present
        if let Some(access_token) = request.access_token() {
            let bearer = format!("Bearer {}", access_token);
            req_builder = req_builder.header("Authorization", bearer);
        }

        // Add all custom headers from the request
        for (key, value) in request.headers() {
            req_builder = req_builder.header(key, value);
        }

        // Add JSON body if present (for POST, PUT, PATCH requests)
        if let Some(body) = request.body_json() {
            req_builder = req_builder.json(body);
        }

        // Send the request
        let response = req_builder.send().await?;
        response.error_for_status_ref()?;

        // Deserialize and return the response
        let result: T = response.json().await?;
        Ok(result)
    }
}
