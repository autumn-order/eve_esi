//! # EVE Online ESI Client Builder
//!
//! This module provides the [`Client`] struct for interacting with the EVE Online ESI (EVE Stable Infrastructure) API.
//!
//! For details on the usage of an [`Client`], see the [client module docs](crate::client).
//!
//! ## References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! ## Features
//! - Set a user agent
//! - Configure [`Client`] for OAuth2 using `client_id`, `client_secret`, and `callback_url` methods
//! - Override the default JWT key cache & refresh settings used to validate OAuth2 tokens & override
//!   the default endpoint URLs with a custom [`Config`] using the [`ClientBuilder::config`] method.
//!
//! ## Builder Methods
//! | Method           | Purpose                                 |
//! | ---------------- | --------------------------------------- |
//! | `new`            | Create a builder for the Client      |
//! | `build`          | Build the Client                     |
//! | `config`         | Override the default config             |
//! | `reqwest_client` | Override default reqwest client         |
//! | `user_agent`     | User agent to identify HTTP requests    |
//! | `client_id`      | EVE OAuth2 client ID                    |
//! | `client_secret`  | EVE OAuth2 client secret                |
//! | `callback_url`   | EVE OAuth2 callback URL                 |
//!
//! ## Usage
//! ```
//! use eve_esi::Client;
//!
//! // Set a user agent used to identify the application making ESI requests
//! let esi_client = Client::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com; +https://github.com/your/repository)")
//!     .build()
//!     .expect("Failed to build Client");
//! ```
//!
//! ## Warning
//! EVE Online's ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
//! Include application name, version, and contact information in your user agent string.
//!
//! Example: `"MyApp/1.0 (contact@example.com; +https://github.com/your/repository)"`

use std::sync::Arc;

use log::warn;

use crate::client::ClientRef;
use crate::config::Config;
use crate::error::Error;
use crate::oauth2::jwk::cache::JwtKeyCache;
use crate::Client;

/// Builder for configuring and constructing an [`Client`].
///
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct ClientBuilder {
    // Base Settings
    /// Config used to override default settings
    pub(crate) config: Option<Config>,
    /// Overrides the default reqwest HTTP client if set to Some()
    pub(crate) reqwest_client: Option<reqwest::Client>,
    /// User agent used for default reqwest client if no client is provided
    pub(crate) user_agent: Option<String>,

    // OAuth2 Settings
    /// Client ID used to identify an EVE Online application
    pub(crate) client_id: Option<String>,
    /// Client secret used to identify an EVE Online application
    pub(crate) client_secret: Option<String>,
    /// URL users are redirected to after the EVE Online login process
    pub(crate) callback_url: Option<String>,
}

impl ClientBuilder {
    /// Creates a new [`ClientBuilder`]
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`ClientBuilder`]: Instance to modify Client setting using setter methods
    pub fn new() -> Self {
        Self {
            // Base settings
            config: None,
            user_agent: None,
            reqwest_client: None,

            // OAuth2 settings
            client_id: None,
            client_secret: None,
            callback_url: None,
        }
    }

    /// Builds the Client
    ///
    /// For a full overview, features, and usage examples, see the [module-level documentation](self).
    ///
    /// # Returns
    /// - [`Client`]: Instance to interface with EVE Online ESI & OAuth2 endpoints.
    ///
    /// # Error
    /// Returns an [`Error`] if:
    /// - There is a user issue related to building an [`oauth2::Client`] usually due to
    ///   missing OAuth2 settings on [`ClientBuilder`] or invalid URLs configured by a custom [`Config`].
    /// - There is an internal issue building a default [`reqwest::Client`]
    /// - There is an internal issue building a default [`Config`]
    pub fn build(self) -> Result<Client, Error> {
        let mut builder = self;

        // Create a default Config if one is not provided
        let config = match builder.config.take() {
            Some(config) => config,
            None => Config::new()?,
        };

        // Setup a reqwest client
        // Will create a reqwest client with default settings & provided user_agent if builder.reqwest_client is none
        let reqwest_client =
            get_or_default_reqwest_client(builder.reqwest_client.take(), &builder.user_agent)?;

        // Build an OAuth2 client if any OAuth2 settings are configured
        //
        // setup_oauth_client return an error if one setting is configured but another
        // is not as all 3 are required for OAuth2.
        let oauth_client = if builder.client_id.is_some()
            || builder.client_secret.is_some()
            || builder.callback_url.is_some()
        {
            Some(builder.setup_oauth_client(&config)?)
        } else {
            None
        };

        // Setup JWT key cache
        let jwt_key_cache = JwtKeyCache::new(&config);

        // Build ClientRef
        let client_ref = ClientRef {
            reqwest_client,
            esi_url: config.esi_url,
            esi_validate_token_before_request: config.esi_validate_token_before_request,

            // OAuth2
            oauth2_client: oauth_client,
            jwt_key_cache: jwt_key_cache,
            jwt_issuers: config.jwt_issuers,
            jwt_audience: config.jwt_audience,
        };

        // Wrap ClientRef in Client
        Ok(Client {
            inner: Arc::new(client_ref),
        })
    }

    /// Overrides the default [`Client`] settings with a custom config
    ///
    /// If no config is provided to the [`ClientBuilder`], the default settings will be used.
    /// This method allows one to provide a config to override the default settings, for details
    /// on usage & options see the [config module documentation](super::config).
    ///
    /// # Arguments
    /// - `config` ([`Config`]): config used to override default [`Client`] settings
    ///
    /// # Returns
    /// - [`ClientBuilder`]: instance with the updated config
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// Override the default [`reqwest::Client`] used by [`Client`]
    ///
    /// Use this to configure the HTTP client used by [`Client`] with your
    /// own preferred settings.
    ///
    /// You can create and configure a reqwest client using the [`reqwest::Client::builder`] method.
    ///
    /// # Warning
    /// The [`ClientBuilder::user_agent`] method will not be applied in the
    /// event that a custom reqwest client is provided, instead you should
    /// set the user agent on the provided [`reqwest::Client`] prior to calling
    /// this method.
    ///
    /// # Arguments
    /// - `client` ([`reqwest::Client`]): An HTTP client used to make requests to
    ///   EVE Online's API endpoints.
    ///
    /// # Returns
    /// - [ClientBuilder]: Instance with the configured reqwest client
    pub fn reqwest_client(mut self, client: reqwest::Client) -> Self {
        self.reqwest_client = Some(client);
        self
    }

    /// Sets the user agent for the Client.
    ///
    /// This method configures the user agent string used by the default reqwest HTTP client created by
    /// the [`Client`] if a custom reqwest client is not provided with the [`Self::reqwest_client`]
    /// method.
    ///
    /// The user agent string is used to identify the client making requests to the EVE Online API.
    /// A proper user agent should include an app name, version, and contact information.
    ///
    /// Example: `"MyApp/1.0 (contact@example.com; +https://github.com/your/repository)"`
    ///
    /// # Warning
    ///
    /// The user agent set by this method will not be applied to the reqwest client provided by the
    /// [`Self::reqwest_client`] method, instead you should set it on the reqwest client you provide prior.
    ///
    /// EVE Online's ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
    ///
    /// # Arguments
    /// - `user_agent` (`&str`): User agent used to identify your application
    ///   when making ESI requests. For example: `"MyApp/1.0 (contact@example.com; +https://github.com/your/repository)"`.
    ///
    /// # Returns
    /// - [`ClientBuilder`]: instance with updated user agent configuration.
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    /// Sets the OAuth2 client ID for authentication with EVE Online SSO.
    ///
    /// This method configures the client ID required for OAuth2 authentication.
    /// You must register your application with EVE Online developers to get a client ID.
    /// <https://developers.eveonline.com/applications>
    ///
    /// # Warning
    /// To enable OAuth2 authentication, you must set `client_id`, `client_secret`, and `callback_url` before calling `.build()`.
    ///
    /// # Arguments
    /// - `client_id` ([`String`]): The OAuth2 client ID obtained from the EVE Online developer portal.
    ///
    /// # Returns
    /// - [`ClientBuilder`]: instance with updated client ID configuration.
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    /// Sets the OAuth2 client secret for authentication with EVE Online SSO.
    ///
    /// This method configures the client secret required for OAuth2 authentication.
    /// You must register your application with EVE Online developers to get a client secret.
    /// <https://developers.eveonline.com/applications>
    ///
    /// # Warning
    /// To enable OAuth2 authentication, you must set `client_id`, `client_secret`, and `callback_url` before calling `.build()`.
    ///
    /// # Arguments
    /// - `client_secret` ([`String`]): The OAuth2 client secret obtained from the EVE Online developer portal.
    ///
    /// # Returns
    /// - [`ClientBuilder`]: instance with updated client secret configuration.
    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = Some(client_secret.to_string());
        self
    }

    /// Sets the callback URL for authentication with EVE Online SSO.
    ///
    /// This method configures the callback URL required for OAuth2 authentication when the user is redirected back to your application.
    /// Ensure that the callback URL matches the one set in your EVE Online developer portal application.
    /// <https://developers.eveonline.com/applications>
    ///
    /// # Warning
    /// To enable OAuth2 authentication, you must set `client_id`, `client_secret`, and `callback_url` before calling `.build()`.
    ///
    /// # Arguments
    /// - `callback_url` ([`String`]): The callback URL which matches the one set in your EVE Online developer portal application.
    ///
    /// # Returns
    /// - [`ClientBuilder`] instance with updated callback URL configuration.
    pub fn callback_url(mut self, callback_url: &str) -> Self {
        self.callback_url = Some(callback_url.to_string());
        self
    }
}

/// Utility function that creates a default [`reqwest::Client`] if no client is provided
///
/// Used with the [`ClientBuilder::build`] method to create a default [`reqwest::Client`] with
/// provided user agent if a reqwest custom client has not been provided.
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
/// - [`reqwest::Client`]: Either a default reqwest client or the provided one.
///
/// # Errors
/// - [`EsiError`]: If the default [`reqwest::Client`] fails to build
fn get_or_default_reqwest_client(
    client: Option<reqwest::Client>,
    user_agent: &Option<String>,
) -> Result<reqwest::Client, Error> {
    if user_agent.is_some() && client.is_some() {
        let message = "user_agent is set on `ClientBuilder` but so is reqwest_client. The user_agent will not be applied and should be instead applied to the provided reqwest client if not done so already.";

        warn!("{}", message);
    }

    match client {
        Some(client) => Ok(client),
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
    use crate::{constant::DEFAULT_ESI_URL, ConfigError};

    /// Test default values of the `ClientBuilder`.
    ///
    /// # Setup
    /// - Create an [`ClientBuilder`] with the default values
    ///
    /// # Assertions
    /// - Assert default values are set as expected
    #[test]
    fn test_default_builder_values() {
        // Create an ClientBuilder with the default values
        let builder = ClientBuilder::new();

        // Assert default values are set as expected
        assert!(builder.config.is_none());
        assert!(builder.reqwest_client.is_none());
        assert!(builder.user_agent.is_none());
        assert!(builder.client_id.is_none());
        assert!(builder.client_secret.is_none());
        assert!(builder.callback_url.is_none());
    }

    /// Test setter methods of the [`ClientBuilder`].
    ///
    /// # Setup
    /// - Create a custom [`reqwest::Client`]
    /// - Create a custom [`JwtKeyCache`]
    /// - Creates an [`Client`] with all builder setter methods used
    ///
    /// # Assertions
    /// - Assert base settings are set as expected
    /// - Assert OAuth2 settings are set as expected
    #[test]
    fn test_builder_setter_methods() {
        let custom_reqwest_client = reqwest::Client::new();
        let custom_config = Config::new().expect("Failed to create a default Config");

        let builder = ClientBuilder::new()
            // Base settings
            .config(custom_config)
            .user_agent("MyApp/1.0 (contact@example.com)")
            .reqwest_client(custom_reqwest_client)
            // OAuth2 settings
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("http://localhost:8000/callback");

        // Assert base values are set
        assert!(builder.config.is_some());
        assert!(builder.reqwest_client.is_some());
        assert_eq!(
            builder.user_agent,
            Some("MyApp/1.0 (contact@example.com)".to_string())
        );

        // Assert OAuth2 values are set
        assert_eq!(builder.client_id, Some("client_id".to_string()));
        assert_eq!(builder.client_secret, Some("client_secret".to_string()));
        assert_eq!(
            builder.callback_url,
            Some("http://localhost:8000/callback".to_string())
        );
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
        let result = ClientBuilder::new()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build();

        // Assert result is Ok
        assert!(result.is_ok());
        let client = result.unwrap();

        // Assert default ESI_URL is set and oauth client is none
        assert_eq!(client.inner.esi_url, DEFAULT_ESI_URL);
        assert!(client.inner.oauth2_client.is_none());
    }

    /// Test successful build with OAuth configuration.
    ///
    /// # Setup
    /// - Creates an ESI client builder with OAuth configuration.
    ///
    /// # Assertions
    /// - Assert result is ok
    /// - Assert oauth client was initialized
    #[test]
    fn test_successful_build_with_oauth() {
        // Test building with OAuth configuration
        let result = ClientBuilder::new()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("http://localhost:8080/callback")
            .build();

        // Assert result is ok
        assert!(result.is_ok());

        // Assert oauth client was initialized
        let client = result.unwrap();
        assert!(client.inner.oauth2_client.is_some());
    }

    /// Test build with a custom config overriding defaults
    ///
    /// # Setup
    /// - Create a config overriding default ESI URL
    /// - Creates an ESI client with the custom config
    ///
    /// # Assertions
    /// - Assert default ESI URL has been overridden
    #[test]
    fn test_build_with_custom_config() {
        // Create a config overriding default ESI URL
        let config = Config::builder()
            .esi_url("https://example.com")
            .build()
            .expect("Failed to create a default Config");

        // Create an ESI client with the custom config
        let result = ClientBuilder::new()
            .config(config)
            .build()
            .expect("Failed to build Client");

        // Assert default ESI URL has been overridden
        assert_ne!(result.inner.esi_url, DEFAULT_ESI_URL);
    }

    /// Test failed build due to partial OAuth configuration.
    ///
    /// # Setup
    /// - Creates an ESI client builder with only the client_id set.
    ///
    /// # Assertions
    /// - Assert result is error
    /// - Assert error is of type ConfigError::MissingClientSecret
    #[test]
    fn test_build_with_partial_oauth_config() {
        // Test that providing only client_id without the other OAuth params fails
        let result = ClientBuilder::new()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .build();

        // Assert result is error
        assert!(result.is_err());

        // Assert error is of type ConfigError::MissingClientSecret
        assert!(matches!(
            result,
            Err(Error::ConfigError(ConfigError::MissingClientSecret))
        ));
    }
}

#[cfg(test)]
mod get_or_default_reqwest_client_tests {
    use crate::builder::get_or_default_reqwest_client;

    /// Ensures a [`reqwest::Client`] is returned when a reqwest client & user agent is provided
    ///
    /// # Test Setup
    /// - Build a custom [`reqwest::Client`]
    /// - Call function with a custom [`reqwest::Client`] and providing a `user_agent`
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
    /// - Call function with `client` set as None and providing a `user_agent`
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
