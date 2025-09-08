//! # EVE Online OAuth2 Client
//!
//! Allows the Client to authenticate with the EVE Online API using OAuth2
//! using the provided client ID, client secret, and callback URL.
//!
//! This module uses the [`oauth2`](https://crates.io/crates/oauth2) crate to configure
//! the OAuth2 client for the Client.
//!
//! This client is only used internally by the [`Client`](crate::Client).
//!
//! - See [module-level documentation](super) for a higher level overview and usage example
//! - See [ClientBuilder docs](crate::builder) for instructions on setting up OAuth2 for the eve_esi crate.

use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    Client, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
    RedirectUrl, RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
};

use crate::builder::ClientBuilder;
use crate::config::Config;
use crate::error::{ConfigError, Error};

/// OAuth2 client type for [`Client`](crate::Client)
///
/// This type represents a client from the oauth2 library which is used
/// within the [`Client`](crate::Client) to authenticate with the EVE Online API using OAuth2.
///
/// This is intended only for internal use by the [`Client`](crate::Client).
pub(crate) type OAuth2Client = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

impl ClientBuilder {
    /// Sets up the OAuth2 client for the [`Client`](crate::Client).
    ///
    /// This method configures the OAuth2 client with the provided client ID, client secret, and callback URL.
    ///
    /// This is intended only for internal use by the [`Client`](crate::Client).
    ///
    /// # Arguments
    /// - `self` ([`ClientBuilder`]): Builder used to set the `client_id`, `client_secret`, and `callback_url`
    /// - `config` (&[`Config`]): Config used to set the EVE Online OAuth2 endpoint URLs
    ///
    /// # Returns
    /// - [`OAuth2Client`]: Instance with configured settings from [`Config`]
    ///
    /// # Errors
    /// - [`OAuthError`]: Error if either the client ID, client secret, or callback URL is missing or
    ///   the callback URL is incorrectly formatted.
    /// - [`OAuthConfigError`]: Error if the auth URL or token URL has been changed from default and
    ///   is incorrectly formatted.
    pub(crate) fn setup_oauth_client(self, config: &Config) -> Result<OAuth2Client, Error> {
        // Get client_id & client_secret
        let client_id = match self.client_id.clone() {
            Some(id) => id.clone(),
            None => return Err(Error::ConfigError(ConfigError::MissingClientId)),
        };
        let client_secret = match self.client_secret.clone() {
            Some(secret) => secret.clone(),
            None => return Err(Error::ConfigError(ConfigError::MissingClientSecret)),
        };

        // Parse URLs
        let callback_url = match self.callback_url.clone() {
            Some(url) => url.clone(),
            None => return Err(Error::ConfigError(ConfigError::MissingCallbackUrl)),
        };
        let redirect_url = match RedirectUrl::new(callback_url) {
            Ok(url) => url,
            Err(_) => return Err(Error::ConfigError(ConfigError::InvalidCallbackUrl)),
        };

        // Create OAuth2 Client
        let client = BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(config.auth_url.clone())
            .set_token_uri(config.token_url.clone())
            .set_redirect_uri(redirect_url);

        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{ConfigError, Error};
    use crate::Client;

    /// Tests the successful build of the OAuth2 client for the [`Client`]
    ///
    /// # Test Setup
    /// - Build an Client with all OAuth2 client related setter methods set
    ///
    /// # Assertions
    /// - Assert result is Ok
    #[test]
    fn test_success() {
        // Create an Client config with all oauth client related setter methods
        let result = Client::builder()
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("http://localhost:8080/callback")
            .build();

        // Assert result is an Ok
        assert!(result.is_ok())
    }

    /// Tests the attempting to initialize an Client for oauth2 with a missing client ID
    ///
    /// # Test Setup
    /// - Creates an ESI client with OAuth2 configured
    ///
    /// # Assertions
    /// - Verifies that the error response is ConfigError::MissingClientId
    #[test]
    fn test_missing_client_id() {
        let result = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_secret("client_secret")
            .callback_url("http://localhost:8080/callback")
            .build();

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(Error::ConfigError(ConfigError::MissingClientId)) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingClientId"),
        }
    }

    /// Tests the attempting to initialize an Client for oauth2 with a missing client secret
    ///
    /// # Test Setup
    /// - Creates an ESI client with the client_secret not set.
    ///
    /// # Assertions
    /// - Verifies that the error response is ConfigError::MissingClientSecret
    #[test]
    fn test_missing_client_secret() {
        let result = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .callback_url("http://localhost:8080/callback")
            .build();

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(Error::ConfigError(ConfigError::MissingClientSecret)) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingClientSecret"),
        }
    }

    /// Tests the attempting initialize an Client for oauth2 with a missing callback_url
    ///
    /// # Test Setup
    /// - Creates an ESI client with the callback_url not set.
    ///
    /// # Assertions
    /// - Verifies that the error response is ConfigError::MissingCallbackUrl
    #[test]
    fn test_missing_callback_url() {
        // Create an ESI client
        let result = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .build();

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(Error::ConfigError(ConfigError::MissingCallbackUrl)) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingCallbackUrl"),
        }
    }

    /// Tests the attempting initialize an Client for oauth2 with an invalid callback_url
    ///
    /// # Test Setup
    /// - Creates an ESI client with the callback_url set to an invalid URL.
    ///
    /// # Assertions
    /// - Verifies that the error response is ConfigError::InvalidCallbackUrl
    #[test]
    fn test_invalid_callback_url() {
        let result = Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("invalid_url") // Invalid URL
            .build();

        assert!(result.is_err());
        match result {
            Err(Error::ConfigError(ConfigError::InvalidCallbackUrl)) => {}
            _ => panic!("Expected InvalidCallbackUrl error"),
        }
    }
}
