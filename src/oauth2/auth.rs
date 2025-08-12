//! OAuth2 authentication methods for EVE Online SSO.
//!
//! See the [module-level documentation](super) for an overview and usage example.

use crate::error::OAuthError;
use crate::EsiClient;

use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};

use crate::model::oauth2::AuthenticationData;

impl EsiClient {
    /// Generates a login URL and state string for initiating the EVE Online OAuth2 authentication process.
    ///
    /// This method constructs the URL that users should visit to begin authentication with EVE Online SSO.
    /// After successful authentication, EVE Online will redirect to the specified callback URL (`redirect_url`),
    /// allowing your application to access the granted user information and permissions.
    ///
    /// # Arguments
    /// - `redirect_url`: The callback URL to which EVE Online will redirect after authentication. This must match the redirect URI registered in your EVE developer application.
    /// - `scopes`: A vector of scope strings representing the permissions your application is requesting. These must match the scopes configured in your EVE developer application.
    ///
    /// # Returns
    /// Returns a [`AuthenticationData`](crate::model::oauth::AuthenticationData) struct containing:
    /// - `login_url`: The URL users should visit to authenticate.
    /// - `state`: A unique state string for CSRF protection.
    ///
    /// # Errors
    /// Returns an [`EveEsiError`](crate::error::EveEsiError) if:
    /// - The client_id or client_secret is missing from the esi_client configuration.
    /// - The provided `redirect_url` is invalid or improperly formatted.
    /// - There is an error constructing the OAuth2 URLs.
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
    ) -> Result<AuthenticationData, OAuthError> {
        let client_id = match self.client_id.clone() {
            Some(id) => id.clone(),
            None => return Err(OAuthError::MissingClientId),
        };
        let client_secret = match self.client_secret.clone() {
            Some(secret) => secret.clone(),
            None => return Err(OAuthError::MissingClientSecret),
        };
        let callback_url = match self.callback_url.clone() {
            Some(url) => url.clone(),
            None => return Err(OAuthError::MissingCallbackUrl),
        };

        let auth_url = match AuthUrl::new(self.eve_auth_url.clone()) {
            Ok(url) => url,
            Err(_) => return Err(OAuthError::InvalidAuthUrl),
        };
        let token_url = match TokenUrl::new(self.eve_auth_token_url.clone()) {
            Ok(url) => url,
            Err(_) => return Err(OAuthError::InvalidTokenUrl),
        };
        let redirect_url = match RedirectUrl::new(callback_url) {
            Ok(url) => url,
            Err(_) => return Err(OAuthError::InvalidCallbackUrl),
        };

        let client = BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

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
    /// Tests the successful generation of an OAuth2 login URL and CSRF state token.
    ///
    /// # Test Setup
    /// - Creates an ESI client with a mock client_id and client_secret
    /// - Configures a redirect URL and requests public_data scope
    /// - Calls the initiate_oauth_login method to generate authentication data
    ///
    /// # Assertions
    /// - Verifies that the generated state token has a non-zero length,
    ///   confirming that proper CSRF protection is in place
    #[test]
    fn test_successful_login_url() {
        let callback_url = "http://localhost:8080/callback";

        let esi_client = crate::EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url(callback_url)
            .build()
            .expect("Failed to build EsiClient");

        let scopes = crate::oauth2::ScopeBuilder::new().public_data().build();

        let auth_data = esi_client.initiate_oauth_login(scopes).unwrap();

        assert!(auth_data.state.len() > 0);
    }

    /// Tests the successful generation of an OAuth2 login URL and CSRF state token.
    ///
    /// # Test Setup
    /// - Creates an ESI client with a mock client_secret and the client_id set to None
    /// - Configures a redirect URL and requests public_data scope
    /// - Calls the initiate_oauth_login method to generate authentication data
    ///
    /// # Assertions
    /// - Verifies that the error response is EsiError::MissingClientId
    #[test]
    fn test_missing_client_id() {
        let callback_url = "http://localhost:8080/callback";

        let esi_client = crate::EsiClient::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_secret("client_secret")
            .callback_url(callback_url)
            .build()
            .expect("Failed to build EsiClient");

        let scopes = crate::oauth2::ScopeBuilder::new().public_data().build();

        let result = esi_client.initiate_oauth_login(scopes);

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(crate::error::OAuthError::MissingClientId) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingClientId"),
        }
    }
}
