//! OAuth2 authentication methods for EVE Online SSO.
//!
//! See the [module-level documentation](super) for an overview and usage example.

use crate::error::{EsiError, OAuthError};
use crate::EsiClient;

use oauth2::{CsrfToken, Scope};

use crate::model::oauth2::AuthenticationData;

impl EsiClient {
    /// Generates a login URL and state string for initiating the EVE Online OAuth2 authentication process.
    ///
    /// This method constructs the URL that users should visit to begin authentication with EVE Online SSO.
    /// After successful authentication, EVE Online will redirect to the callback URL (`callback_url`) specified
    /// in your `EsiClient` configuration with an authorization code to receive an access token (See [crate::oauth2::token::get_token]).
    ///
    /// # Arguments
    /// - `scopes`: A vector of scope strings representing the permissions your application is requesting. These must match the scopes configured in your EVE developer application.
    ///
    /// # Returns
    /// Returns a [`AuthenticationData`](crate::model::oauth2::AuthenticationData) struct containing:
    /// - `login_url`: The URL users should visit to authenticate.
    /// - `state`: A unique state string for CSRF protection.
    ///
    /// # Errors
    /// Returns an [`EveEsiError`](crate::error::EveEsiError) if:
    /// - The client_id, client_secret, and callback_url is missing from the esi_client configuration
    ///   which results in an `OAuthClientNotConfigured` error.
    ///
    /// # Notes
    /// - The `state` string should be stored and verified upon callback to protect against CSRF attacks.
    ///
    /// # Example
    /// ```
    /// let esi_client = eve_esi::EsiClient::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback")
    ///     .build()
    ///     .expect("Failed to build EsiClient");
    ///
    /// let scopes = eve_esi::oauth2::ScopeBuilder::new()
    ///     .public_data()
    ///     .build();
    /// let auth_data = esi_client
    ///     .initiate_oauth_login(scopes)
    ///     .unwrap();
    ///
    /// println!("Login URL: {}", auth_data.login_url);
    /// ```
    pub fn initiate_oauth_login(
        &self,
        scopes: Vec<String>,
    ) -> Result<AuthenticationData, EsiError> {
        let client = if let Some(ref client) = self.oauth_client {
            client
        } else {
            return Err(EsiError::OAuthError(OAuthError::OAuth2NotConfigured));
        };

        let scopes: Vec<Scope> = scopes.into_iter().map(Scope::new).collect();

        let (eve_oauth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .url();

        Ok(AuthenticationData {
            login_url: eve_oauth_url.to_string(),
            state: csrf_token.secret().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{EsiError, OAuthError};
    use crate::oauth2::ScopeBuilder;

    /// Tests the successful generation of an OAuth2 login URL and CSRF state token.
    ///
    /// # Test Setup
    /// - Creates an ESI client with a client_id, client_secret, and callback_url
    /// - Configure scopes with the public_data scope
    /// - Calls the initiate_oauth_login method to generate authentication data
    ///
    /// # Assertions
    /// - Verifies that the generated state token has a non-zero length,
    ///   confirming that proper CSRF protection is in place
    #[test]
    fn test_successful_login_url() {
        let esi_client = crate::EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("http://localhost:8080/callback")
            .build()
            .expect("Failed to build EsiClient");

        let scopes = ScopeBuilder::new().public_data().build();

        let auth_data = esi_client.initiate_oauth_login(scopes).unwrap();

        assert!(auth_data.state.len() > 0);
    }

    /// Tests attempting to initiate an OAuth2 login without configuring the client ID, client secret, or callback URL.
    ///
    /// # Test Setup
    /// - Creates an ESI client without setting the client_id, client_secret, or callback_url
    /// - Configure scopes with the public_data scope
    /// - Calls the initiate_oauth_login method to generate authentication data
    ///
    /// # Assertions
    /// - Verifies that the error response is EsiError::OAuthError(OAuthError::OAuthClientNotConfigured)
    #[test]
    fn test_oauth_client_not_configured() {
        let esi_client = crate::EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build EsiClient");

        let scopes = ScopeBuilder::new().public_data().build();

        let result = esi_client.initiate_oauth_login(scopes);

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(EsiError::OAuthError(OAuthError::OAuth2NotConfigured)) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::OAuthError(OAuthError::OAuth2NotConfigured)"),
        }
    }
}
