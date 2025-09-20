#![warn(missing_docs)]

//! ## EVE ESI
//!
//! A thread-safe, asynchronous client which provides methods & types for interaction with
//! [EVE Online's ESI](https://developers.eveonline.com/api-explorer) &
//! [EVE Online's single sign-on (SSO)](https://developers.eveonline.com/docs/services/sso/).
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//! - <https://developers.eveonline.com/docs/services/sso/>
//!
//! ## Quickstart
//!
//! ### ESI Client
//!
//! - [Creating a basic ESI client for public ESI endpoints](crate::client)
//! - [Building an ESI client for OAuth2 & authenticated ESI endpoints](crate::builder)
//! - [Overriding an ESI client's defaults](crate::config)
//!
//! ### Making ESI Requests
//!
//! - [Making requests to public ESI endpoints](crate::endpoints)
//! - [Making requests to authenticated ESI endpoints](crate::endpoints)
//!
//! ### Single Sign-On (OAuth2)
//!
//! - [Building scopes to request during login](crate::scope)
//! - [Creating a login URL for single sign-on (OAuth2)](crate::oauth2::login)
//! - [Fetching an access token](crate::oauth2::token)
//! - [Validating an access token](crate::oauth2::token)
//! - [Refreshing an access token](crate::oauth2::token)
//!
//! ### Error Types
//!
//! - [Runtime errors](crate::error::Error)
//! - [Configuration errors](crate::error::ConfigError)
//! - [OAuth2 runtime errors](crate::oauth2::error::OAuthError)
//!
//! ### Custom Endpoints
//!
//! - [Adding custom ESI endpoints](crate::esi)
//!
//! ## Usage
//!
//! Create a new ESI Client instance and request public information about a corporation from ESI.
//!
//! ```no_run
//! // esi_client is asynchronous, #[tokio::main] allows for making the main function async
//! // You would ideally use esi_client with an async web framework like Axum as shown in examples
//! #[tokio::main]
//! async fn main() {
//!     // Build a new ESI Client with the builder method
//!     let esi_client = eve_esi::Client::builder()
//!     // Always set a user agent to identify your application
//!         .user_agent("MyApp/1.0 (contact@example.com; +https://github.com/your/repository)")
//!         .build()
//!         .expect("Failed to build Client");
//!
//!     // Get information about the corporation The Order of Autumn (id: 98785281)
//!     let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();
//!     println!("Corporation name: {}", corporation.name);
//! }
//! ```
//!
//! # Logging
//!
//! This library uses the [`log`](https://crates.io/crates/log) crate for logging. To capture log output,
//! applications using this library should initialize a logger implementation like `env_logger`,
//! `simple_logger`, or any other implementation of the `log` crate's facade.
//!
//! ## Log Levels
//!
//! - **Error**: Used for failures that prevent successful API calls
//! - **Warn**: Used for potential issues that don't prevent operation but could be problematic
//! - **Info**: Used for successful API calls and important client state changes
//! - **Debug**: Used for detailed information about API call parameters and responses
//! - **Trace**: Used for very detailed debugging information
//!
//! ## Example with env_logger
//!
//! ```no_run
//! // Set RUST_LOG environment variable to control log levels
//! // e.g., RUST_LOG=eve_esi=debug,info
//!
//! // Initialize env_logger
//! env_logger::init();
//!
//! // Now logs from eve_esi will be captured
//! let esi_client = eve_esi::Client::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .build()
//!     .expect("Failed to build Client");
//! ```

#[macro_use]
mod logging;

pub mod builder;
pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod esi;
pub mod model;
pub mod oauth2;
pub mod scope;

pub use crate::builder::ClientBuilder;
pub use crate::client::Client;
pub use crate::config::{Config, ConfigBuilder};
pub use crate::error::{ConfigError, Error};
pub use crate::oauth2::error::OAuthError;
pub use crate::scope::ScopeBuilder;

mod constant;

#[cfg(test)]
mod tests;
