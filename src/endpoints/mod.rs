//! Methods for interacting with EVE Online's ESI API.
//!
//! This module provides access to the various categories of endpoints available in the EVE Online ESI API.
//!
//! # References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//!
//! # Modules
//!
//! - [`alliances`](crate::endpoints::alliances)
//! - [`characters`](crate::endpoints::characters)
//! - [`corporations`](crate::endpoints::corporations)
//!
//! # Example
//! ```no_run
//! #[tokio::main]
//! async fn main() {
//!     let esi_client = eve_esi::EsiClient::builder()
//!         .user_agent("MyApp/1.0 (contact@example.com)")
//!         .build()
//!         .expect("Failed to build EsiClient");
//!
//!     // Get information about the corporation The Order of Autumn (id: 98785281)
//!     let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();
//!     println!("Corporation name: {}", corporation.name);
//! }
//! ```

pub mod alliance;
pub mod character;
pub mod corporation;
pub mod endpoints;
