//! Methods for interacting with EVE Online's ESI API.
//!
//! This module provides access to the various categories of endpoints available in the EVE Online ESI API.
//!
//! # References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//!
//! # Modules
//!
//! - [`alliance`]: Alliance Endpoints for EVE Online's ESI API.
//! - [`character`]: Character Endpoints for EVE Online's ESI API.
//! - [`corporation`]: Corporation Endpoints for EVE Online's ESI API.
//!
//! # Example
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

pub mod alliance;
pub mod character;
pub mod corporation;
pub mod endpoints;
