//! # EVE ESI Request Module
//!
//! This module provides types and methods for making requests to EVE Online's ESI API.
//!
//! ## Core Components
//!
//! - **[`EsiRequest`]**: Builder for configuring ESI requests with headers, authentication, and body data
//! - **[`CacheStrategy`]**: Type-safe caching strategy with `chrono::DateTime` for conditional requests
//! - **[`CachedResponse`]**: Response type that handles 304 Not Modified responses
//! - **[`Language`]**: Type-safe enum for ESI language headers
//! - **[`EsiApi`]**: Request executor that handles authentication and HTTP communication
//!
//! ## Basic Usage
//!
//! ```no_run
//! use eve_esi::{Client, EsiRequest};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct ServerStatus {
//!     players: i32,
//!     server_version: String,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("MyApp/1.0 (contact@example.com)")?;
//!
//! // Simple request
//! let request = client.esi().new_request::<ServerStatus>("/status/");
//! let status = request.send().await?;
//! println!("Players online: {}", status.players);
//! # Ok(())
//! # }
//! ```
//!
//! ## Cached Requests
//!
//! Use [`CacheStrategy`] with [`EsiRequest::send_cached`] to handle 304 Not Modified responses:
//!
//! ```no_run
//! use eve_esi::{Client, EsiRequest, CacheStrategy, CachedResponse};
//! use chrono::{DateTime, Utc};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct ServerStatus {
//!     players: i32,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("MyApp/1.0")?;
//!
//! // Make request with caching
//! let last_check: DateTime<Utc> = Utc::now();
//! let request = client.esi().new_request::<ServerStatus>("/status/");
//! let response = request
//!     .send_cached(CacheStrategy::IfModifiedSince(last_check))
//!     .await?;
//!
//! if response.is_not_modified() {
//!     println!("Data hasn't changed");
//! } else if let CachedResponse::Fresh(data) = response {
//!     println!("Fresh data: {} players", data.players);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Authenticated Requests
//!
//! ```no_run
//! use eve_esi::{Client, EsiRequest};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Character {
//!     name: String,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("MyApp/1.0")?;
//! let access_token = "your_oauth2_token";
//!
//! let request = client.esi().new_request::<Character>("/characters/12345/")
//!     .with_access_token(access_token)
//!     .with_required_scopes(vec!["publicData".to_string()]);
//!
//! let character = request.send().await?;
//! # Ok(())
//! # }
//! ```

// Submodules
mod client;
mod request;
mod response;

#[cfg(test)]
mod tests;

// Re-export public API
pub use client::EsiApi;
pub use request::{CacheStrategy, EsiRequest, Language};
pub use response::{CacheHeaders, CachedResponse, EsiResponse, RateLimitHeaders};

// Internal utilities
mod util;
