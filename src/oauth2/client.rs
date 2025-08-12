//! Configures the OAuth2 client for the EsiClient.
//!
//! This module uses the oauth2 library to configure the OAuth2 client for the EsiClient.
//! Allows the EsiClient to authenticate with the EVE Online API using OAuth2
//! using the provided client ID, client secret, and callback URL.

use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthUrl, Client, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
    RedirectUrl, RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};

use crate::client::EsiClientBuilder;
use crate::error::{EsiError, OAuthError};

/// OAuth2 client type for EsiClient
///
/// This type represents a client from the oauth2 library which is used
/// within the EsiClient to authenticate with the EVE Online API using OAuth2.
///
/// This is intended only for internal use by the EsiClient.
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

impl EsiClientBuilder {
    /// Sets up the OAuth2 client for the EsiClient.
    ///
    /// This method configures the OAuth2 client with the provided client ID, client secret, and callback URL.
    ///
    /// # Errors
    /// - Returns an error if either the client ID, client secret, or callback URL is missing.
    /// - Returns an error if the auth_url, token_url, or redirect_url is incorrectly formatted.
    ///
    /// This is intended only for internal use by the EsiClient.
    pub(crate) fn setup_oauth_client(mut self) -> Result<Self, EsiError> {
        let client_id = match self.client_id.clone() {
            Some(id) => id.clone(),
            None => return Err(EsiError::OAuthError(OAuthError::MissingClientId)),
        };
        let client_secret = match self.client_secret.clone() {
            Some(secret) => secret.clone(),
            None => return Err(EsiError::OAuthError(OAuthError::MissingClientSecret)),
        };
        let callback_url = match self.callback_url.clone() {
            Some(url) => url.clone(),
            None => return Err(EsiError::OAuthError(OAuthError::MissingCallbackUrl)),
        };

        let auth_url = match AuthUrl::new(self.auth_url.clone()) {
            Ok(url) => url,
            Err(_) => return Err(EsiError::OAuthError(OAuthError::InvalidAuthUrl)),
        };
        let token_url = match TokenUrl::new(self.token_url.clone()) {
            Ok(url) => url,
            Err(_) => return Err(EsiError::OAuthError(OAuthError::InvalidTokenUrl)),
        };
        let redirect_url = match RedirectUrl::new(callback_url) {
            Ok(url) => url,
            Err(_) => return Err(EsiError::OAuthError(OAuthError::InvalidCallbackUrl)),
        };

        let client = BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);
        self.oauth_client = Some(client);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{EsiError, OAuthError};
    use crate::EsiClient;

    /// Tests the attempting to initialize an EsiClient for oauth2 with a missing client ID
    ///
    /// # Test Setup
    /// - Creates an ESI client with the client_id not set.
    ///
    /// # Assertions
    /// - Verifies that the error response is EsiError::MissingClientId
    #[test]
    fn test_missing_client_id() {
        let result = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_secret("client_secret")
            .callback_url("http://localhost:8080/callback")
            .build();

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(EsiError::OAuthError(OAuthError::MissingClientId)) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingClientId"),
        }
    }

    /// Tests the attempting to initialize an EsiClient for oauth2 with a missing client secret
    ///
    /// # Test Setup
    /// - Creates an ESI client with the client_secret not set.
    ///
    /// # Assertions
    /// - Verifies that the error response is EsiError::MissingClientSecret
    #[test]
    fn test_missing_client_secret() {
        let result = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .callback_url("http://localhost:8080/callback")
            .build();

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(EsiError::OAuthError(OAuthError::MissingClientSecret)) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingClientSecret"),
        }
    }

    /// Tests the attempting initialize an EsiClient for oauth2 with a missing callback_url
    ///
    /// # Test Setup
    /// - Creates an ESI client with the callback_url not set.
    ///
    /// # Assertions
    /// - Verifies that the error response is EsiError::MissingCallbackUrl
    #[test]
    fn test_missing_callback_url() {
        let result = EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .build();

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(EsiError::OAuthError(OAuthError::MissingCallbackUrl)) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingCallbackUrl"),
        }
    }
}
