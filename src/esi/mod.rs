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

use crate::Client;

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
}
