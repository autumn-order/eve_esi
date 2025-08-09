pub mod error;
pub mod model;

mod auth;
mod endpoints;
mod esi;

/// A client for interacting with EVE Online's ESI (EVE Swagger Interface) API.
///
/// This client provides methods for making authenticated and unauthenticated requests to the ESI API.
/// It handles authentication with EVE Online's OAuth2 implementation and provides convenience methods
/// for accessing various ESI endpoints.
pub struct EsiClient {
    reqwest_client: reqwest::Client,
    client_id: Option<String>,
    client_secret: Option<String>,
    pub esi_url: String,
    pub eve_auth_url: String,
    pub eve_auth_token_url: String,
}

/// Creates a new ESI client with default configuration.
///
/// # Arguments
/// - `user_agent` - The User-Agent header to use for requests. CCP requires a descriptive user agent
///                  that includes your application name, version, and contact information.
///
/// # Returns
/// A new `EsiClient` instance with default ESI and authentication URLs.
///
/// # Example
/// ```
/// use eve_esi::EsiClient;
///
/// let esi_client = EsiClient::new("MyApp/1.0 (contact@example.com)");
/// ```
impl EsiClient {
    pub fn new(user_agent: &str) -> Self {
        Self {
            reqwest_client: reqwest::Client::builder()
                .user_agent(user_agent)
                .build()
                .unwrap(),
            client_id: None,
            client_secret: None,
            esi_url: "https://esi.evetech.net/latest".to_string(),
            eve_auth_url: "https://login.eveonline.com/v2/oauth/".to_string(),
            eve_auth_token_url: "https://login.eveonline.com/v2/oauth/token".to_string(),
        }
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
    pub fn set_client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
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
    pub fn set_client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }
}

#[deprecated(since = "0.3.0", note = "Use EsiClient instead")]
pub type Client = EsiClient;
