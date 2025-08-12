//! # EVE Online ESI API Client
//!
//! This module provides the [`EsiClient`] struct for interacting with the EVE Online ESI (EVE Stable Infrastructure) API.
//!
//! ## Features
//! - Set up user agent, client ID, client secret, and callback URL
//! - Make authenticated and unauthenticated requests to ESI endpoints
//! - Handles OAuth2 authentication with EVE Online SSO
//!
//! ## Key Methods
//! | Method         | Purpose                                 |
//! | -------------- | --------------------------------------- |
//! | `user_agent`   | Set the HTTP user agent                 |
//! | `client_id`    | Set OAuth2 client ID                    |
//! | `client_secret`| Set OAuth2 client secret                |
//! | `callback_url` | Set OAuth2 callback URL                 |
//!
//! ## References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! ## Example
//! ```
//! use eve_esi::EsiClient;
//!
//! let esi_client = EsiClient::new()
//!     .user_agent("MyApp/1.0 (contact@example.com)");
//! ```
//!
//! ## Warning
//! EVE ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
//! Include application name, version, and contact information in your user agent string.
//!
//! ## Deprecated
//! The `Client` type alias is deprecated. Use [`EsiClient`] directly.

/// The main client for interacting with EVE Online's ESI (EVE Stable Infrastructure) API.
///
/// Use this struct to configure authentication and make requests to ESI endpoints.
/// For a full overview, features, and usage examples, see the [module-level documentation](self).
pub struct EsiClient {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
    pub(crate) callback_url: Option<String>,
    /// The base URL for the EVE Online ESI API.
    pub esi_url: String,
    /// The URL for EVE Online oauth2 authorization.
    pub eve_auth_url: String,
    /// The URL for the EVE Online oauth2 token.
    pub eve_auth_token_url: String,
}

impl EsiClient {
    /// Creates a new ESI client with default configuration.
    ///
    /// # Returns
    /// A new `EsiClient` instance with default ESI and authentication URLs.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::new()
    ///     .user_agent("MyApp/1.0 (contact@example.com)");
    /// ```
    ///
    /// # Warning
    /// EVE ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
    ///
    /// Include application name, version, and contact information.
    ///
    /// You can do so with `esi_client.user_agent("MyApp/1.0 (contact@example.com)");`
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
            client_id: None,
            client_secret: None,
            callback_url: None,
            esi_url: "https://esi.evetech.net/latest".to_string(),
            eve_auth_url: "https://login.eveonline.com/v2/oauth/authorize".to_string(),
            eve_auth_token_url: "https://login.eveonline.com/v2/oauth/token".to_string(),
        }
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
    /// The `EsiClient` instance with updated user agent configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::new()
    ///     .user_agent("MyApp/1.0 (contact@example.com)");
    /// ```
    ///
    /// # Warning
    /// EVE ESI API requires setting a proper user agent. Failure to do so may result in rate limiting or API errors.
    ///
    /// Include application name, version, and contact information.
    ///
    /// Example: "MyApp/1.0 (contact@example.com)"
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.reqwest_client = reqwest::Client::builder()
            .user_agent(user_agent.to_string())
            .build()
            .unwrap();
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
    /// The `EsiClient` instance with updated client ID configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::new()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id");
    /// ```
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
    /// The `EsiClient` instance with updated client secret configuration.
    ///
    /// # Example
    /// ```
    /// use eve_esi::EsiClient;
    ///
    /// let esi_client = EsiClient::new()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret");
    /// ```
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
    /// let esi_client = EsiClient::new()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback");
    /// ```
    pub fn callback_url(mut self, callback_url: &str) -> Self {
        self.callback_url = Some(callback_url.to_string());
        self
    }
}

/// Deprecated: Use [`EsiClient`] instead.
///
/// This type alias is retained for backward compatibility and will be removed in a future release.
/// Please update your code to use [`EsiClient`] directly.
#[deprecated(note = "Use `EsiClient` instead")]
pub type Client = EsiClient;
