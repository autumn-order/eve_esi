//! # EVE Online ESI Client Builder
//!
//! This module provides the [`EsiClient`] struct for interacting with the EVE Online ESI (EVE Stable Infrastructure) API.
//!
//! ## Features
//! - Set up user agent, client ID, client secret, and callback URL
//! - Make authenticated and unauthenticated requests to ESI endpoints
//! - Handles OAuth2 authentication with EVE Online SSO
//!
//! ## Builder Methods
//! | Method         | Purpose                                 |
//! | -------------- | --------------------------------------- |
//! | `builder`      | Create a builder for the EsiClient      |
//! | `build`        | Build the EsiClient                     |
//! | `user_agent`   | Set the HTTP user agent                 |
//! | `client_id`    | Set OAuth2 client ID                    |
//! | `client_secret`| Set OAuth2 client secret                |
//! | `callback_url` | Set OAuth2 callback URL                 |
//! | `esi_url`      | Set a custom URL for the ESI API        |
//! | `auth_url`     | Set a custom URL for EVE OAuth2         |
//! | `token_url`    | Set a custom URL for EVE OAuth2 token   |
//! | `jwk_url`      | Set a custom URL for EVE OAuth2 JWKS    |
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

use crate::error::EsiError;
use crate::oauth2::client::OAuth2Client;
use crate::EsiClient;

/// Builder for configuring and constructing an `EsiClient`.
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiClientBuilder {
    pub(crate) oauth_client: Option<OAuth2Client>,
    pub(crate) user_agent: Option<String>,
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
    pub(crate) callback_url: Option<String>,
    pub(crate) esi_url: String,
    pub(crate) auth_url: String,
    pub(crate) token_url: String,
    pub(crate) jwk_url: String,
}

impl EsiClientBuilder {
    /// Creates a new EsiClientBuilder
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    pub fn new() -> Self {
        Self {
            oauth_client: None,
            user_agent: None,
            client_id: None,
            client_secret: None,
            callback_url: None,
            esi_url: "https://esi.evetech.net/latest".to_string(),
            auth_url: "https://login.eveonline.com/v2/oauth/authorize".to_string(),
            token_url: "https://login.eveonline.com/v2/oauth/token".to_string(),
            jwk_url: "https://login.eveonline.com/oauth/jwks".to_string(),
        }
    }

    /// Builds the EsiClient
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    pub fn build(self) -> Result<EsiClient, EsiError> {
        let mut client_builder = reqwest::Client::builder();
        if let Some(ref user_agent) = self.user_agent {
            client_builder = client_builder.user_agent(user_agent.clone());
        }
        let reqwest_client = client_builder.build()?;

        let mut builder = self;
        if builder.client_id.is_some()
            || builder.client_secret.is_some()
            || builder.callback_url.is_some()
        {
            builder = builder.setup_oauth_client()?;
        }

        Ok(EsiClient {
            reqwest_client,
            oauth_client: builder.oauth_client,
            esi_url: builder.esi_url,
            jwk_url: builder.jwk_url,
            jwt_keys_cache: Mutex::new(None),
            jwt_keys_cache_ttl: 3600, // Default: 1 hour cache TTL
        })
    }

    /// Sets the user agent for the EsiClient.
    ///
    /// This method configures the user agent string used by the reqwest HTTP client.
    /// The user agent string is used to identify the client making requests to the EVE Online API.
    /// A proper user agent should include an app name, version, and contact information.
    /// Example: "MyApp/1.0 (contact@example.com)"
    ///
    /// # Arguments
    /// - `user_agent` - The user agent string to be used by the reqwest HTTP client.
    ///
    /// # Returns
    /// The `EsiClientBuilder` instance with updated user agent configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    ///
    /// # Warning
    /// EVE ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
    ///
    /// Include application name, version, and contact information.
    ///
    /// Example: "MyApp/1.0 (contact@example.com)"
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    /// Sets the OAuth2 client ID for authentication with EVE Online SSO.
    ///
    /// This method configures the client ID required for OAuth2 authentication.
    /// You must register your application with EVE Online developers to get a client ID.
    /// https://developers.eveonline.com/applications
    ///
    /// # Arguments
    /// - `client_id` - The OAuth2 client ID obtained from the EVE Online developer portal.
    ///
    /// # Returns
    /// The `EsiClientBuilder` instance with updated client ID configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    ///
    /// Note: For OAuth2 authentication, you must set `client_id`, `client_secret`, and `callback_url` before calling `.build()`.
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    /// Sets the OAuth2 client secret for authentication with EVE Online SSO.
    ///
    /// This method configures the client secret required for OAuth2 authentication.
    /// You must register your application with EVE Online developers to get a client secret.
    /// https://developers.eveonline.com/applications
    ///
    /// # Arguments
    /// - `client_secret` - The OAuth2 client secret obtained from the EVE Online developer portal.
    ///
    /// # Returns
    /// The `EsiClientBuilder` instance with updated client secret configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    ///
    /// Note: For OAuth2 authentication, you must set `client_id`, `client_secret`, and `callback_url` before calling `.build()`.
    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = Some(client_secret.to_string());
        self
    }

    /// Sets the callback URL for authentication with EVE Online SSO.
    ///
    /// This method configures the callback URL required for OAuth2 authentication when the user is redirected back to your application.
    /// Ensure that the callback URL matches the one set in your EVE Online developer portal application.
    /// https://developers.eveonline.com/applications
    ///
    /// # Arguments
    /// - `callback_url` - The callback URL which matches the one set in your EVE Online developer portal application.
    ///
    /// # Returns
    /// The `EsiClient` instance with updated callback URL configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    ///
    /// Note: For OAuth2 authentication, you must set `client_id`, `client_secret`, and `callback_url` before calling `.build()`.
    pub fn callback_url(mut self, callback_url: &str) -> Self {
        self.callback_url = Some(callback_url.to_string());
        self
    }

    /// Sets the EVE Online ESI base URL to a custom URL.
    ///
    /// This method configures the base URL for EVE Online ESI.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `esi_url` - The EVE Online API base URL.
    ///
    /// # Returns
    /// The `EsiClientBuilder` instance with updated EVE Online API base URL configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .esi_url("https://esi.evetech.net/latest")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    pub fn esi_url(mut self, esi_url: &str) -> Self {
        self.esi_url = esi_url.to_string();
        self
    }

    /// Sets the EVE Online oauth2 authorize URL to a custom URL.
    ///
    /// This method configures the authorize URL for EVE Online oauth2.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `auth_url` - The EVE Online oauth2 authorize URL.
    ///
    /// # Returns
    /// The `EsiClientBuilder` instance with updated EVE Online oauth2 authorize URL configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .esi_url("https://login.eveonline.com/v2/oauth/authorize")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    pub fn auth_url(mut self, auth_url: &str) -> Self {
        self.auth_url = auth_url.to_string();
        self
    }

    /// Sets the EVE Online oauth2 token URL to a custom URL.
    ///
    /// This method configures the token URL for EVE Online oauth2 to a custom URL.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `token_url` - The EVE Online oauth2 token URL.
    ///
    /// # Returns
    /// The `EsiClientBuilder` instance with updated EVE Online oauth2 token URL configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .token_url("https://login.eveonline.com/v2/oauth/token")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    pub fn token_url(mut self, token_url: &str) -> Self {
        self.token_url = token_url.to_string();
        self
    }

    /// Sets the EVE Online JWK URI to a custom URL.
    ///
    /// This method configures the JWK URI for EVE Online OAuth2 to a custom URL.
    /// This is generally used for tests using a mock server with crates such as
    /// [mockito](https://crates.io/crates/mockito) to avoid actual ESI API calls.
    ///
    /// # Arguments
    /// - `jwk_url` - The EVE Online JWK URI.
    ///
    /// # Returns
    /// The `EsiClientBuilder` instance with updated EVE Online JWK URI configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .jwk_url("https://login.eveonline.com/oauth/jwks")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    /// ```
    pub fn jwk_url(mut self, jwk_url: &str) -> Self {
        self.jwk_url = jwk_url.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::OAuthError;

    /// Test default values of the `EsiClientBuilder`.
    #[test]
    fn test_default_builder_values() {
        let builder = EsiClientBuilder::new();

        // Check default values
        assert_eq!(builder.esi_url, "https://esi.evetech.net/latest");
        assert_eq!(
            builder.auth_url,
            "https://login.eveonline.com/v2/oauth/authorize"
        );
        assert_eq!(
            builder.token_url,
            "https://login.eveonline.com/v2/oauth/token"
        );
        assert_eq!(builder.jwk_url, "https://login.eveonline.com/oauth/jwks");
        assert!(builder.user_agent.is_none());
        assert!(builder.client_id.is_none());
        assert!(builder.client_secret.is_none());
        assert!(builder.callback_url.is_none());
        assert!(builder.oauth_client.is_none());
    }

    /// Test setter methods of the `EsiClientBuilder`.
    #[test]
    fn test_builder_setter_methods() {
        let builder = EsiClientBuilder::new()
            .esi_url("https://example.com")
            .auth_url("https://auth.example.com")
            .token_url("https://token.example.com")
            .jwk_url("https://jwk.example.com")
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("https://callback.example.com");

        // Check updated values
        assert_eq!(builder.esi_url, "https://example.com");
        assert_eq!(builder.auth_url, "https://auth.example.com");
        assert_eq!(builder.token_url, "https://token.example.com");
        assert_eq!(builder.jwk_url, "https://jwk.example.com");
        assert_eq!(
            builder.user_agent,
            Some("MyApp/1.0 (contact@example.com)".to_string())
        );
        assert_eq!(builder.client_id, Some("client_id".to_string()));
        assert_eq!(builder.client_secret, Some("client_secret".to_string()));
        assert_eq!(
            builder.callback_url,
            Some("https://callback.example.com".to_string())
        );
    }

    /// Test build without user agent.
    ///
    /// The builder allows building without a user agent, but it's not recommended
    /// This test just verifies it doesn't fail
    #[test]
    fn test_build_without_user_agent() {
        let result = EsiClientBuilder::new().build();

        assert!(result.is_ok());
    }

    /// Test successful build with minimal configuration.
    #[test]
    fn test_successful_build_minimal() {
        // Test building with just the required user_agent
        let result = EsiClientBuilder::new().user_agent("Test App").build();

        assert!(result.is_ok());
        let client = result.unwrap();
        assert_eq!(client.esi_url, "https://esi.evetech.net/latest");
        assert!(client.oauth_client.is_none());
    }

    /// Test successful build with OAuth configuration.
    #[test]
    fn test_successful_build_with_oauth() {
        // Test building with OAuth configuration
        let result = EsiClientBuilder::new()
            .user_agent("Test App")
            .client_id("test_client_id")
            .client_secret("test_client_secret")
            .callback_url("https://example.com/callback")
            .build();

        assert!(result.is_ok());
        let client = result.unwrap();
        assert!(client.oauth_client.is_some());
    }

    /// Test failed build due to partial OAuth configuration.
    #[test]
    fn test_build_with_partial_oauth_config() {
        // Test that providing only client_id without the other OAuth params fails
        let result = EsiClientBuilder::new()
            .user_agent("Test App")
            .client_id("test_client_id")
            .build();

        assert!(result.is_err());
        match result {
            Err(EsiError::OAuthError(OAuthError::MissingClientSecret)) => {}
            _ => panic!("Expected MissingClientSecret error"),
        }
    }

    /// Ensure that the builder correctly transfers configuration to the client.
    #[test]
    fn test_builder_to_client_configuration_transfer() {
        let custom_esi_url = "https://custom-esi.example.com";
        let custom_jwk_url = "https://custom-jwk.example.com";

        let client = EsiClientBuilder::new()
            .user_agent("Test App")
            .esi_url(custom_esi_url)
            .jwk_url(custom_jwk_url)
            .build()
            .expect("Failed to build client");

        // Verify the client received the configured values from the builder
        assert_eq!(client.esi_url, custom_esi_url);
        assert_eq!(client.jwk_url, custom_jwk_url);
    }
}
