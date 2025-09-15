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
//! ## Modules
//! - [`esi`]: Provides the [`EsiApi`] struct implementing request methods on the [`crate::Client`]
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
//!     // Setup a basic Client
//!     let esi_client = eve_esi::Client::builder()
//!         .user_agent("MyApp/1.0 (contact@example.com; +https://github.com/your/repository)")
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

pub mod authenticated;
pub mod esi;
pub mod public;

pub use esi::EsiApi;

mod util;
