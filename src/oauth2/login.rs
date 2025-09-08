//! EVE Online OAuth2 Login
//!
//! Provides the method to create a login URL to begin the EVE Online single sign-on (SSO) process.
//! See the [OAuth2Api::login_url] method for details.
//!
//! See the [module-level documentation](super) for an overview of EVE Online OAuth2 and usage example.

use oauth2::{CsrfToken, Scope};

use crate::error::{Error, OAuthError};
use crate::model::oauth2::AuthenticationData;
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Generates a login URL and state string for initiating the EVE Online OAuth2 authentication process.
    ///
    /// This method constructs the URL that begins the login process for EVE Online SSO (single sign-on) or also known as OAuth2.
    /// After successful authentication, EVE Online will redirect the user to the callback URL (`callback_url`) specified
    /// in your [`Client`](crate::Client) configuration with an authorization code used to request an access token with the
    /// [crate::oauth2::OAuth2Api::get_token] method.
    ///
    /// # Arguments
    /// - `scopes` (Vec<[`String`]>): A vec of scope strings representing the permissions your application is requesting.
    ///   These must match the scopes configured in your EVE developer application.
    ///
    /// # Returns
    /// Returns a [`AuthenticationData`](crate::model::oauth2::AuthenticationData) struct containing:
    /// - `login_url` ([`String`]): The URL users should visit to authenticate.
    /// - `state` ([`String`]): A unique state string used for CSRF protection.
    ///
    /// # Errors
    /// Returns an [`EsiError`] if:
    /// - The `client_id`, `client_secret`, and `callback_url` is missing from the [`Client`](crate::Client) configuration
    ///   which results in an [`OAuthError::OAuth2NotConfigured`] error.
    ///
    /// # Example
    /// ```
    /// // Configure Client for OAuth2 with a client_id, client_secret, and callback_url
    /// let esi_client = eve_esi::Client::builder()
    ///     .user_agent("MyApp/1.0 (contact@example.com)")
    ///     .client_id("client_id")
    ///     .client_secret("client_secret")
    ///     .callback_url("http://localhost:8080/callback")
    ///     .build()
    ///     .expect("Failed to build Client");
    ///
    /// // Build scopes requesting only publicData
    /// let scopes = eve_esi::oauth2::ScopeBuilder::new()
    ///     .public_data()
    ///     .build();
    ///
    /// // Create a login URL
    /// let auth_data = esi_client
    ///     .oauth2()
    ///     .login_url(scopes)
    ///     .expect("Failed to create a login url");
    ///
    /// // Print the created login URL
    /// println!("Login URL: {}", auth_data.login_url);
    /// ```
    pub fn login_url(&self, scopes: Vec<String>) -> Result<AuthenticationData, Error> {
        // Retrieve the OAuth2 client from the Client
        let client = match &self.client.inner.oauth2_client {
            Some(client) => client,
            // Returns an error if the OAuth2 client is not found due to it not having been configured when
            // building the Client.
            None => return Err(Error::OAuthError(OAuthError::OAuth2NotConfigured)),
        };

        // Convert the Vec<String> of scopes into Vec<Scope>
        let scopes: Vec<Scope> = scopes.into_iter().map(Scope::new).collect();

        // Create the login url & a CSRF state code
        let (eve_oauth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .url();

        // Return login url & state code
        Ok(AuthenticationData {
            login_url: eve_oauth_url.to_string(),
            state: csrf_token.secret().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{Error, OAuthError};
    use crate::oauth2::ScopeBuilder;

    /// Tests the successful generation of an OAuth2 login URL and CSRF state token.
    ///
    /// # Test Setup
    /// - Configure [`Client`](crate::Client) for OAuth2 with a client_id, client_secret, and callback_url
    /// - Build scopes requesting only publicData
    ///
    /// # Assertions
    /// - Verifies that the generated state token has a non-zero length,
    ///   confirming that proper CSRF protection is in place
    #[test]
    fn test_successful_login_url() {
        // Configure Client for OAuth2 with a client_id, client_secret, and callback_url
        let esi_client = crate::Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .client_id("client_id")
            .client_secret("client_secret")
            .callback_url("http://localhost:8080/callback")
            .build()
            .expect("Failed to build Client");

        // Build scopes requesting only publicData
        let scopes = ScopeBuilder::new().public_data().build();

        // Get a login URL
        let result = esi_client.oauth2().login_url(scopes);

        // Assert result is ok
        assert!(result.is_ok());
    }

    /// Ensures the proper error is received when attempting to generate a login url without configuring OAuth2
    ///
    /// # Test Setup
    /// - Create an [`Client`](crate::Client) without setting the client_id, client_secret, or callback_url
    /// - Build scopes requesting only publicData
    ///
    /// # Assertions
    /// - Assert result is an error
    /// - Ensure error is of type EsiError::OAuthError(OAuthError::OAuth2NotConfigured)
    #[test]
    fn test_oauth_client_not_configured() {
        // Create an ESI client without setting the client_id, client_secret, or callback_url
        let esi_client = crate::Client::builder()
            .user_agent("MyApp/1.0 (contact@example.com)")
            .build()
            .expect("Failed to build Client");

        // Build scopes requesting only publicData
        let scopes = ScopeBuilder::new().public_data().build();

        // Get a login URL
        let result = esi_client.oauth2().login_url(scopes);

        // Assert result is an error
        assert!(result.is_err());

        // Ensure error is of type EsiError::OAuthError(OAuthError::OAuth2NotConfigured)
        match result {
            Err(Error::OAuthError(OAuthError::OAuth2NotConfigured)) => {},
            err => panic!("Expected EsiError::OAuthError(OAuthError::OAuth2NotConfigured), instead received: {:#?}", err),
        }
    }
}
