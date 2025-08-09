use crate::error::EsiError;
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
    /// static USER_AGENT: &str = "APPLICATION_NAME/1.0 (example@example.com)";
    /// let mut esi_client = eve_esi::Client::new(&USER_AGENT)
    ///     .set_client_id("example".to_string())
    ///     .set_client_secret("example".to_string());
    ///
    /// let redirect_url = "http://localhost:8080/callback".to_string();
    /// let scopes = eve_esi::oauth2::ScopeBuilder::new()
    ///     .public_data()
    ///     .build();
    /// let auth_data = esi_client
    ///     .initiate_oauth_login(redirect_url, scopes)
    ///     .unwrap();
    ///
    /// println!("Login URL: {}", auth_data.login_url);
    /// ```
    pub fn initiate_oauth_login(
        self,
        redirect_url: String,
        scopes: Vec<String>,
    ) -> Result<AuthenticationData, EsiError> {
        fn convert_scopes(scopes: Vec<String>) -> Vec<Scope> {
            scopes.iter().map(|s| Scope::new(s.clone())).collect()
        }

        let client_id = match self.client_id {
            Some(id) => id.clone(),
            None => return Err(EsiError::MissingClientId),
        };
        let client_secret = match self.client_secret {
            Some(secret) => secret.clone(),
            None => return Err(EsiError::MissingClientSecret),
        };

        let auth_url = AuthUrl::new(self.eve_auth_url).map_err(|_| {
            EsiError::ParseError(format!(
                "Failed to parse the EVE Online AuthUrl.\n\
                You can change the url by setting the `eve_auth_url` field in your `Client` configuration."

            ))
        })?;
        let token_url = TokenUrl::new(self.eve_auth_token_url).map_err(|_| {
            EsiError::ParseError(format!(
                "Failed to parse the EVE Online TokenUrl.\n\
                You can change the url by setting the `eve_auth_token_url` field in your `Client` configuration."
            ))
        })?;
        let redirect_url = RedirectUrl::new(redirect_url).map_err(|_| {
            EsiError::ParseError(format!(
                "The provided redirect_url is invalid or improperly formatted. Please ensure it is a valid URL and matches the redirect URI registered in your EVE Online developer application (https://developers.eveonline.com/applications)."
            ))
        })?;

        let client = BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        let scopes = convert_scopes(scopes);

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
        static USER_AGENT: &str = "APPLICATION_NAME/1.0 (example@example.com)";
        let esi_client = crate::EsiClient::new(&USER_AGENT)
            .set_client_id("example".to_string())
            .set_client_secret("example".to_string());

        let redirect_url = "http://localhost:8080/callback".to_string();
        let scopes = crate::oauth2::ScopeBuilder::new().public_data().build();

        let auth_data = esi_client
            .initiate_oauth_login(redirect_url, scopes)
            .unwrap();

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
        static USER_AGENT: &str = "APPLICATION_NAME/1.0 (example@example.com)";
        let mut esi_client = crate::EsiClient::new(&USER_AGENT);

        esi_client.client_id = None;
        esi_client.client_secret = Some("example".to_string());

        let redirect_url = "http://localhost:8080/callback".to_string();
        let scopes = crate::oauth2::ScopeBuilder::new().public_data().build();

        let result = esi_client.initiate_oauth_login(redirect_url, scopes);

        match result {
            Ok(_) => {
                panic!("Expected Err");
            }
            Err(crate::error::EsiError::MissingClientId) => {
                assert!(true);
            }
            Err(_) => panic!("Expected EsiError::MissingClientId"),
        }
    }
}
