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

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::builder::EsiClientBuilder;
use crate::model::oauth2::EveJwtKeys;
use crate::oauth2::client::OAuth2Client;

type JwtKeyCache = Arc<RwLock<Option<(EveJwtKeys, std::time::Instant)>>>;

/// The main client for interacting with EVE Online's ESI (EVE Stable Infrastructure) API.
///
/// Use this struct to configure authentication and make requests to ESI endpoints.
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiClient {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) oauth_client: Option<OAuth2Client>,
    pub(crate) esi_url: String,
    pub(crate) jwk_url: String,
    /// Cache TTL for JWT keys in seconds.
    pub jwt_keys_cache_ttl: u64,

    // Use an Arc to share the cache & refresh status between threads
    // This is necessary for the background JWT key refresh task spawned when calling the
    // `get_jwt_keys` method in `oauth2/jwk.rs`.
    /// Cache for JWT keys used to validate tokens from EVE Online's OAuth2 API.
    ///
    /// Consider using the [`get_jwt_keys`] method to retrieve the keys from the cache &
    /// automatically refresh them when expired.
    /// Direct modification of this field is typically only for testing purposes.
    pub jwt_keys_cache: JwtKeyCache,
    /// Flag indicating whether a JWT key refresh is currently in progress to prevent concurrent refreshes.
    pub jwt_key_refresh_in_progress: Arc<AtomicBool>,
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
    use crate::constant::{DEFAULT_AUTH_URL, DEFAULT_ESI_URL, DEFAULT_JWK_URL, DEFAULT_TOKEN_URL};

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
        assert_eq!(builder.esi_url, DEFAULT_ESI_URL);
        assert_eq!(builder.auth_url, DEFAULT_AUTH_URL);
        assert_eq!(builder.token_url, DEFAULT_TOKEN_URL);
        assert_eq!(builder.jwk_url, DEFAULT_JWK_URL);
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
