//! # EVE Online ESI Client
//!
//! This module provides the [`Client`] struct for interacting with EVE Online's ESI (EVE Stable Infrastructure) API.
//!
//! ## Features
//! - Make authenticated and unauthenticated requests to ESI endpoints
//! - Handle OAuth2 authentication with EVE Online SSO
//!
//! ## Client Creation
//! The client is created using the builder pattern. See the [`builder`](crate::builder) module for configuration options.
//!
//! ## References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! ## Warning
//! EVE ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
//! Include application name, version, and contact information in your user agent string.
//!
//! Example: `"MyApp/1.0 (contact@example.com)"`
//!
//! ## Example
//! ```
//! // Set a user agent used to identify the application making ESI requests
//! let esi_client = eve_esi::Client::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .build()
//!     .expect("Failed to build Client");
//! ```

use std::sync::Arc;

use crate::builder::ClientBuilder;
use crate::oauth2::client::OAuth2Client;
use crate::oauth2::jwk::cache::JwtKeyCache;

/// The main client for interacting with EVE Online's ESI (EVE Stable Infrastructure) API.
///
/// Use this struct to configure OAuth2 authentication and make requests to ESI endpoints. Uses
/// an [`Arc`] internally for usage across multiple threads.
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
#[derive(Clone)]
pub struct Client {
    /// Inner reference containing the actual client implementation.
    pub(crate) inner: Arc<ClientRef>,
}

/// Reference type containing the actual client implementation.
///
/// This struct is wrapped in an [`Arc`] within the [`Client`] struct.
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub(crate) struct ClientRef {
    // Base settings
    /// HTTP client used to make requests to EVE Online's APIs
    pub(crate) reqwest_client: reqwest::Client,
    /// The base EVE Online ESI API URL
    pub(crate) esi_url: String,
    /// The EVE Online login server URL which represents the expected issuer of tokens
    pub(crate) login_url: String,

    // OAuth2 Settings
    /// OAuth2 client used for accessing EVE Online OAuth2 endpoints
    ///
    /// Will be None if `client_id`, `client_secret`, and `callback_url` have not been
    /// set on the [`Client`] which will result in errors if trying to use OAuth2-related endpoints.
    pub(crate) oauth2_client: Option<OAuth2Client>,
    /// Cache containing JWT keys for validating OAuth2 tokens and fields for coordinating
    /// cache usage & refreshes across threads.
    pub(crate) jwt_key_cache: JwtKeyCache,
    /// The intended audience which JWT tokens will be used with
    pub(crate) jwt_audience: String,
}

impl Client {
    /// Creates a new [`ClientBuilder`]
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the successful minimal build of [`Client::builder`]
    ///
    /// # Setup
    /// - Setup an ClientBuilder using the builder() method
    ///
    /// # Assertions
    /// - Validate that the default values are correct
    /// - Verify that the esi_client has built successfully
    #[test]
    fn test_successful_build_minimal() {
        // Test that builder() returns a valid ClientBuilder
        let builder = Client::builder();

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
