//! # EVE ESI Request Utilities
//!
//! Provides utility methods for making requests to EVE Online's ESI. These
//! methods are used internally by the [`crate::endpoints`] to make requests.
//!
//! Despite the use case intended primarily to be internal, these functions are exported publicly
//! to allow for using the ESI client to make requests to custom ESI routes. This is useful
//! for when this crate hasn't implemented an ESI route yet but you still wish to use the client
//! to make requests to the route.
//!
//! # Methods
//!
//! - [`EsiApi::new]: Creates a new instance of [`EsiApi`]
//! - [`EsiApi::get_from_public_esi`]: Makes an unauthenticated GET request to the ESI API.
//! - [`EsiApi::post_to_public_esi`]: Makes an unauthenticated POST request to the ESI API.
//!
//! # Usage
//!
//! ```no_run
//! use serde::{Serialize, Deserialize};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup a basic Client
//!     let esi_client = eve_esi::Client::builder()
//!         .user_agent("MyApp/1.0 (contact@example.com")
//!         .build()
//!         .expect("Failed to build ESI Client");
//!
//!     // Define the struct to deserialize the ESI response to
//!     #[derive(Serialize, Deserialize)]
//!     pub struct CharacterAffiliation {
//!         pub alliance_id: Option<i64>,
//!         pub character_id: i64,
//!         pub corporation_id: i64,
//!         pub faction_id: Option<i64>,
//!     };
//!
//!     // Define the URL to make the request to
//!     let url = "https://esi.evetech.net/characters/affiliation/";
//!
//!     // Make the request with the earlier defined struct
//!     // - The first type, `<Vec<CharacterAffiliation>`, represents the response body to deserialize
//!     // - The second type, `Vec<i64>`, represents the request body to serialize
//!     let character_ids = vec![2114794365];
//!
//!     let affiliations = esi_client
//!         .esi()
//!         .post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i64>>(&url, &character_ids)
//!         .await;
//! }
//! ```

use serde::{de::DeserializeOwned, Serialize};

use crate::Client;

/// Provides utility methods for making requests EVE Online's ESI endpoints
///
/// See the [module-level documentation](self) for an overview, methods, & usage example.
pub struct EsiApi<'a> {
    pub(crate) client: &'a Client,
}

impl Client {
    /// Access to utility functions to make ESI requests
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    pub fn esi(&self) -> self::EsiApi<'_> {
        self::EsiApi::new(self)
    }
}

impl<'a> EsiApi<'a> {
    /// Creates a new instance of [`EsiApi`]
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `client` (&'a [`Client`]) used for making HTTP requests to EVE Online's ESI & OAuth2
    ///   endpoints and providing the JWT key caching & refresh handling used to validate tokens.
    ///
    /// # Returns
    /// - `Self`: A new instance of [`EsiApi`].
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Makes an unauthenticated GET request to the ESI API.
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]): The ESI API endpoint URL to request.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`
    pub async fn get_from_public_esi<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, reqwest::Error> {
        let reqwest_client = &self.client.inner.reqwest_client;

        let req = reqwest_client.get(url).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }

    /// Makes an unauthenticated POST request to the ESI API.
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]):  The ESI API endpoint URL to request.
    /// - `data` ([`Serialize`]): The data to send in the request body.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`.
    /// - `U` - The type of data to send, which must implement `Serialize`.
    pub async fn post_to_public_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
        &self,
        url: &str,
        data: &U,
    ) -> Result<T, reqwest::Error> {
        let reqwest_client = &self.client.inner.reqwest_client;

        let req = reqwest_client.post(url).json(data).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }
}
