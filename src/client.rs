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

use std::sync::Arc;

use crate::builder::EsiClientBuilder;
use crate::oauth2::client::OAuth2Client;
use crate::oauth2::jwk::cache::JwtKeyCache;

/// The main client for interacting with EVE Online's ESI (EVE Stable Infrastructure) API.
///
/// Use this struct to configure OAuth2 authentication and make requests to ESI endpoints.
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiClient {
    // Base settings
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) esi_url: String,

    // OAuth2 Settings
    /// OAuth2 client used for accessing EVE Online OAuth2 endpoints
    ///
    /// Will be None if client_id, client_secret, and callback_url have not been
    /// set on the EsiClient.
    pub(crate) oauth2_client: Option<OAuth2Client>,
    /// Cache containing JWT keys for validating OAuth2 tokens and fields for coordinating
    /// cache usage & refreshes across threads.
    pub jwt_key_cache: Arc<JwtKeyCache>,
}

impl EsiClient {
    /// Creates a new EsiClientBuilder
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    pub fn builder() -> EsiClientBuilder {
        EsiClientBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the successful minimal build of [`EsiClient::builder`]
    ///
    /// # Setup
    /// - Setup an EsiClientBuilder using the builder() method
    ///
    /// # Assertions
    /// - Validate that the default values are correct
    /// - Verify that the esi_client has built successfully
    #[test]
    fn test_successful_build_minimal() {
        // Test that builder() returns a valid EsiClientBuilder
        let builder = EsiClient::builder();

        // Verify the builder has expected default values
        assert!(builder.user_agent.is_none());
        assert!(builder.client_id.is_none());
        assert!(builder.client_secret.is_none());
        assert!(builder.callback_url.is_none());

        // Verify that the esi_client has built successfully
        let esi_client = builder.build();
        assert!(esi_client.is_ok());

        // Note: More comprehensive tests for the builder pattern are in builder.rs
    }
}
