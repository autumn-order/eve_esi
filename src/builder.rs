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
//! | Method          | Purpose                                 |
//! | --------------- | --------------------------------------- |
//! | `new`           | Create a builder for the EsiClient      |
//! | `build`         | Build the EsiClient                     |
//! | `user_agent`    | User agent to identify HTTP requests    |
//! | `client_id`     | EVE OAuth2 client ID                    |
//! | `client_secret` | EVE OAuth2 client secret                |
//! | `callback_url`  | EVE OAuth2 callback URL                 |
//! | `esi_url`       | ESI API URL                             |
//! | `oauth2_config` | OAuth2 related configuration settings   |
//!
//! ## References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! ## Usage
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

use log::warn;

use crate::constant::DEFAULT_ESI_URL;
use crate::error::EsiError;
use crate::oauth2::config::client::OAuth2Client;
use crate::oauth2::jwk::cache::JwtKeyCache;
use crate::oauth2::OAuth2Config;
use crate::EsiClient;

/// Builder for configuring and constructing an `EsiClient`.
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiClientBuilder {
    pub(crate) esi_url: String,
    pub(crate) user_agent: Option<String>,
    pub(crate) reqwest_client: Option<reqwest::Client>,

    // OAuth2
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
    pub(crate) callback_url: Option<String>,
    pub(crate) oauth_client: Option<OAuth2Client>,
    pub(crate) oauth2_config: Option<OAuth2Config>,
}

impl EsiClientBuilder {
    /// Creates a new EsiClientBuilder
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`EsiClientBuilder`]: Instance to modify EsiClient setting using setter methods
    ///
    /// # Error
    /// - [`EsiError`]: If the default [`OAuth2Config`] cannot be initialized which would be an internal
    ///   error with the eve_esi crate regarding improperly formatted default OAuth2 URLs.
    pub fn new() -> Self {
        Self {
            esi_url: DEFAULT_ESI_URL.to_string(),
            user_agent: None,
            reqwest_client: None,

            // OAuth2
            client_id: None,
            client_secret: None,
            callback_url: None,
            oauth_client: None,
            oauth2_config: None,
        }
    }

    /// Builds the EsiClient
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`EsiClient`]: Instance to interface with EVE Online ESI & OAuth2 endpoints.
    ///
    /// # Error
    /// - [`EsiError`]: If there is an issue building a [`reqwest::Client`],
    ///   [`oauth2::Client`], or there is an internal error building the default
    ///   [`OAuth2Config`].
    pub fn build(self) -> Result<EsiClient, EsiError> {
        let mut builder = self;

        // Setup a reqwest client
        // Will create a reqwest client with default settings & provided user_agent if builder.reqwest_client is none
        let reqwest_client =
            get_or_default_reqwest_client(builder.reqwest_client.take(), &builder.user_agent)?;

        // Initialize default OAuth2 config if none is set
        let oauth2_config = match builder.oauth2_config.take() {
            Some(config) => config,
            None => OAuth2Config::default()?,
        };

        // Build an OAuth2 client if any OAuth2 settings are configured
        //
        // setup_oauth_client return an error if one setting is configured but another
        // is not as all 3 are required for OAuth2.
        if builder.client_id.is_some()
            || builder.client_secret.is_some()
            || builder.callback_url.is_some()
        {
            builder = builder.setup_oauth_client(&oauth2_config)?;
        }

        // Build EsiClient
        Ok(EsiClient {
            reqwest_client,
            esi_url: builder.esi_url,

            // OAuth2oauth2_config
            oauth2_client: builder.oauth_client,
            oauth2_config: oauth2_config,

            // OAuth2 JWT key cache
            jwt_key_cache: Arc::new(JwtKeyCache::new()),
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

    /// Changes OAuth2 config to a custom one to override default OAuth2 settings
    ///
    /// This method allows for overriding the default EVE OAuth2 settings using
    /// a [`OAuth2Config`] struct to set custom values.
    ///
    /// This is used for overriding the EVE OAuth2 API endpoint URLs
    /// for testing purposes or for more precise control over how the JWT keys
    /// used to validate tokens are cached and fetched.
    ///
    /// See [OAuth2 Config module docs](`crate::oauth2::config`) for usage & details.
    ///
    /// # Arguments
    /// - `config` ([`OAuth2Config`]): A struct representing OAuth2 configuration settings
    ///
    /// # Returns
    /// - [EsiClientBuilder]: Instance with updated OAuth2 config
    pub fn oauth2_config(mut self, config: OAuth2Config) -> Self {
        self.oauth2_config = Some(config);
        self
    }

    /// Override the default [`reqwest::Client`] used by [`EsiClient`]
    ///
    /// Use this to configure the HTTP client used by [`EsiClient`] with your
    /// own preferred settings.
    ///
    /// # Warning
    /// The [`EsiClientBuilder::user_agent`] method will not be applied in the
    /// event that a custom reqwest client is provided, instead you should
    /// set the user agent on the provided [`reqwest::Client`] prior to calling
    /// this method.
    ///
    /// # Arguments
    /// - `client` ([`reqwest::Client`]): An HTTP client used to make requests to
    ///   EVE Online's API endpoints.
    ///
    /// # Returns
    /// - [EsiClientBuilder]: Instance with updated reqwest Client
    pub fn reqwest_client(mut self, client: reqwest::Client) -> Self {
        self.reqwest_client = Some(client);
        self
    }
}

/// Utility function that creates a default [`reqwest::Client`] if no client is provided
///
/// Used with the [`EsiClientBuilder::build`] method to create a default [`reqwest::Client`] with
/// provided user agent if a custom client has not been provided.
///
/// Provides a warning if both a custom agent and user agent has been provided as the user agent
/// cannot be set on the provided client, the user agent should be set on the provided client prior
/// instead.
///
/// # Arguments
/// - `client` (Option<[`reqwest::Client`]): Option of a reqwest::Client to determine if a default one
///   should be created and returned.
/// - `user_agent` (&Option<[`reqwest::Client`]): Option of a user agent that will be applied to the
///   default reqwest::Client if no `client` is provided.
///
/// # Returns
/// - [`reqwest::Client`]: Either a default client or the provided one.
///
/// # Errors
/// - [`EsiError`]: If the default [`reqwest::Client`] fails to build
fn get_or_default_reqwest_client(
    client: Option<reqwest::Client>,
    user_agent: &Option<String>,
) -> Result<reqwest::Client, EsiError> {
    match client {
        Some(client) => {
            if user_agent.is_some() {
                #[cfg(not(tarpaulin_include))]
                warn!("user_agent is set on `EsiClientBuilder` but so is reqwest_client, as a result the user_agent will not be applied and should be instead applied to the provided reqwest client if not done so already.");
            }

            Ok(client)
        }
        None => {
            let mut client_builder = reqwest::Client::builder();
            if let Some(agent) = user_agent {
                client_builder = client_builder.user_agent(agent.clone());
            }

            Ok(client_builder.build()?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::OAuthError;

    /// Test default values of the `EsiClientBuilder`.
    ///
    /// # Setup
    /// - Creates an ESI client using the default values
    ///
    /// # Assertions
    /// - Checks that the default values are set correctly
    #[test]
    fn test_default_builder_values() {
        let builder = EsiClientBuilder::new();

        // Check default values
        assert_eq!(builder.esi_url, DEFAULT_ESI_URL);
        assert!(builder.user_agent.is_none());
        assert!(builder.client_id.is_none());
        assert!(builder.client_secret.is_none());
        assert!(builder.callback_url.is_none());
        assert!(builder.oauth_client.is_none());
    }

    /// Test setter methods of the `EsiClientBuilder`.
    ///
    /// # Setup
    /// - Creates an ESI client with all values modified
    ///
    /// # Assertions
    /// - Checks that all setter methods were set correctly
    #[test]
    fn test_builder_setter_methods() {
        let custom_reqwest_client = reqwest::Client::new();

        let builder = EsiClientBuilder::new()
            .esi_url("https://example.com")
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("http://localhost:8000/callback")
            .reqwest_client(custom_reqwest_client);

        // Check updated values
        assert_eq!(builder.esi_url, "https://example.com");
        assert_eq!(
            builder.user_agent,
            Some("MyApp/1.0 (contact@example.com)".to_string())
        );
        assert_eq!(builder.client_id, Some("client_id".to_string()));
        assert_eq!(builder.client_secret, Some("client_secret".to_string()));
        assert_eq!(
            builder.callback_url,
            Some("http://localhost:8000/callback".to_string())
        );
    }

    /// Test build without user agent.
    ///
    /// The builder allows building without a user agent, but it's not recommended
    /// This test just verifies it doesn't fail
    ///
    /// # Setup
    /// - Creates an ESI client builder without a user agent.
    ///
    /// # Assertions
    /// - Verifies that the client was built successfully.
    #[test]
    fn test_build_without_user_agent() {
        let result = EsiClientBuilder::new().build();

        assert!(result.is_ok());
    }

    /// Test successful build with minimal configuration.
    ///
    /// # Setup
    /// - Creates an ESI client builder with minimal configuration.
    ///
    /// # Assertions
    /// - Verifies that the client receives the configured values from the builder.
    #[test]
    fn test_successful_build_minimal() {
        // Test building with just the required user_agent
        let result = EsiClientBuilder::new()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build();

        assert!(result.is_ok());
        let client = result.unwrap();
        assert_eq!(client.esi_url, DEFAULT_ESI_URL);
        assert!(client.oauth2_client.is_none());
    }

    /// Test successful build with OAuth configuration.
    ///
    /// # Setup
    /// - Creates an ESI client builder with OAuth configuration.
    ///
    /// # Assertions
    /// - Verifies that the client receives the configured values from the builder.
    #[test]
    fn test_successful_build_with_oauth() {
        // Test building with OAuth configuration
        let result = EsiClientBuilder::new()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("http://localhost:8080/callback")
            .build();

        assert!(result.is_ok());
        let client = result.unwrap();
        assert!(client.oauth2_client.is_some());
    }

    /// Test failed build due to partial OAuth configuration.
    ///
    /// # Setup
    /// - Creates an ESI client builder with only the client_id set.
    ///
    /// # Assertions
    /// - Verifies that the error response is EsiError::OAuthError(OAuthError::MissingClientSecret)
    #[test]
    fn test_build_with_partial_oauth_config() {
        // Test that providing only client_id without the other OAuth params fails
        let result = EsiClientBuilder::new()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .build();

        assert!(result.is_err());
        match result {
            Err(EsiError::OAuthError(OAuthError::MissingClientSecret)) => {}
            _ => panic!("Expected MissingClientSecret error"),
        }
    }
}

#[cfg(test)]
mod get_or_default_reqwest_client_tests {
    use crate::builder::get_or_default_reqwest_client;

    /// Ensures a [`reqwest::Client`] is returned when a client & user agent is provided
    ///
    /// # Test Setup
    /// - Build a custom reqwest::Client
    /// - Call function with a custom reqwest::Client and providing a user_agent
    ///
    /// # Assert
    /// - Assert result is Ok indicating a reqwest client was returned without issues
    #[test]
    fn test_custom_client_and_agent() {
        let user_agent = "MyApp/1.0 (contact@example.com)".to_string();
        let timeout = std::time::Duration::from_secs(10);

        // Build a reqwest client with custom settings
        let client = reqwest::Client::builder()
            // Always set user_agent when providing a custom client to get_or_default_reqwest_client
            // as the function is unable to modify the provided client further.
            .user_agent(&user_agent)
            .timeout(timeout)
            .build()
            .expect("Failed to build reqwest::Client");

        // Call function
        //
        // The provided agent won't be used but we'll add it to make sure the warning execution path is called
        let result = get_or_default_reqwest_client(Some(client), &Some(user_agent));

        // Assert result is Ok
        assert!(result.is_ok());
    }

    /// Ensures a default [`reqwest::Client`] is returned if no client is provided
    ///
    /// # Test Setup
    /// - Call function with reqwest::Client as None and providing a user_agent
    ///
    /// # Assert
    /// - Assert result is Ok indicating a default reqwest client with default settings has been returned
    #[test]
    fn test_default_with_agent() {
        let result = get_or_default_reqwest_client(None, &Some("Agent".to_string()));

        // Assert result is Ok
        assert!(result.is_ok());
    }
}
