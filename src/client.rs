//! # EVE Online ESI Client
//!
//! This module provides the [`EsiClient`] struct for interacting with the EVE Online ESI (EVE Stable Infrastructure) API.
//!
//! ## Features
//! - Make authenticated and unauthenticated requests to ESI endpoints
//! - Handles OAuth2 authentication with EVE Online SSO
//!
//! ## Client Creation
//! The client is created using the builder pattern. See the [`builder`](crate::builder) module for configuration options.
//!
//! ## References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! ## Example
//! ```
//! use eve_esi::EsiClient;
//!
//! let esi_client = EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .build()
//!     .expect("Failed to build EsiClient");
//! ```
//!
//! ## Warning
//! EVE ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
//! Include application name, version, and contact information in your user agent string.

use tokio::sync::Mutex;

use crate::builder::EsiClientBuilder;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::client::OAuth2Client;

/// The main client for interacting with EVE Online's ESI (EVE Stable Infrastructure) API.
///
/// Use this struct to configure authentication and make requests to ESI endpoints.
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiClient {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) oauth_client: Option<OAuth2Client>,
    pub(crate) esi_url: String,
    pub(crate) jwk_url: String,
    pub(crate) jwt_keys_cache: Mutex<Option<(EveJwtKeys, std::time::Instant)>>,
    pub(crate) jwt_keys_cache_ttl: u64,
}

impl EsiClient {
    /// Creates a new EsiClientBuilder
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    pub fn builder() -> EsiClientBuilder {
        EsiClientBuilder::new()
    }
}
