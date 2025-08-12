#![warn(missing_docs)]

//! # EVE ESI
//! Rust API wrapper for interaction with [EVE Online's ESI](https://developers.eveonline.com/api-explorer).
//! See the [README](https://github.com/hyziri/eve_esi/blob/main/README.md) for more examples and details.
//!
//! # References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! # Usage
//!
//! Create a new EsiClient instance and request public information about a character from ESI.
//!
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
//!
//! Make certain you set the user agent as demonstrated above, ensure it includes contact email in case there are any issues with your ESI requests.

pub mod error;
pub mod model;
pub mod oauth2;

pub use crate::client::EsiClient;

mod client;
mod endpoints;
mod esi;
