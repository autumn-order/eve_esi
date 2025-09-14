//! # EVE Online ESI Client
//!
//! This module provides the [`Client`] struct for accessing ESI endpoints, logging in with EVE Online
//! SSO (single sign-on) using OAuth2, & caching & refreshing JWT keys to validate tokens.
//!
//! ## Usage
//!
//! Creating a basic default ESI client with a user agent
//!
//! ```rust
//! // Set a user_agent to identify your application when making requests
//! let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)";
//!
//! // Create a basic ESI client with user_agent
//! let esi_client = eve_esi::Client::new(user_agent).expect("Failed to build Client");
//! ```
//!
//! To build an ESI client for OAuth2 & authenticated ESI routes, please see the [`crate::builder`] module docs.
//!
//! ## Warning
//! EVE ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
//! Include application name, version, and contact information in your user agent string.
//!
//! Example: `"MyApp/1.0 (contact@example.com; +https://github.com/your/repo)"`

use std::sync::Arc;

use crate::builder::ClientBuilder;
use crate::oauth2::client::OAuth2Client;
use crate::oauth2::jwk::cache::JwtKeyCache;
use crate::Error;

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

    // OAuth2 Settings
    /// OAuth2 client used for accessing EVE Online OAuth2 endpoints
    ///
    /// Will be None if `client_id`, `client_secret`, and `callback_url` have not been
    /// set on the [`Client`] which will result in errors if trying to use OAuth2-related endpoints.
    pub(crate) oauth2_client: Option<OAuth2Client>,
    /// Cache containing JWT keys for validating OAuth2 tokens and fields for coordinating
    /// cache usage & refreshes across threads.
    pub(crate) jwt_key_cache: JwtKeyCache,
    /// The EVE Online login server which represents the expected issuer of tokens
    pub(crate) jwt_issuers: Vec<String>,
    /// The intended audience which JWT tokens will be used with
    pub(crate) jwt_audience: String,
}

impl Client {
    /// Creates a basic [`Client`] with a user agent
    ///
    /// This Client can only be used for public ESI routes. To build an ESI client for
    /// OAuth2 & authenticated ESI routes, please see [crate::builder] module documentation.
    ///
    /// For an overview & usage example, see the [module-level documentation](self)
    ///
    /// # Arguments
    /// - `user_agent` (`&str`): User agent used to identify your application
    ///   when making ESI requests. For example: `"MyApp/1.0 (contact@example.com; +https://github.com/your/repository)"`.
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - [`Client`]: Instance to interface with EVE Online ESI endpoints
    /// - [`Error`]: An error if there is an issue with the default client config
    pub fn new(user_agent: &str) -> Result<Client, Error> {
        ClientBuilder::new().user_agent(user_agent).build()
    }

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

    /// Test successful build of a basic client using [`Client::new`]
    ///
    /// # Test Setup
    /// - Set an example user agent
    ///
    /// # Assertions
    /// - Assert result is ok
    #[test]
    fn test_basic_client_build() {
        // Set an example user agent
        let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repo)";

        // Create a new basic client
        let result = Client::new(user_agent);

        // Assert result is ok
        assert!(result.is_ok())
    }

    /// Test the successful setup of an ESI client using the [`Client::builder`] method
    ///
    /// # Test Setup
    /// - Create a ClientBuilder with default settings
    ///
    /// # Assertions
    /// - Assert builder has expected default values
    /// - Assert build method builds a Client successfully
    #[test]
    fn test_successful_builder_minimal() {
        // Create a ClientBuilder with default settings
        let builder = Client::builder();

        // Assert builder has expected default values
        assert!(builder.user_agent.is_none());
        assert!(builder.client_id.is_none());
        assert!(builder.client_secret.is_none());
        assert!(builder.callback_url.is_none());

        // Assert build method builds a Client successfully
        let esi_client = builder.build();
        assert!(esi_client.is_ok());

        // Note: More comprehensive tests for the builder pattern are in builder.rs
    }
}
