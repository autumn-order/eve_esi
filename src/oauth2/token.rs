//! Methods to retrieve & validate tokens from EVE Online's OAuth2 API.

use crate::error::{EsiError, OAuthError};
use crate::EsiClient;

use oauth2::basic::BasicTokenType;
use oauth2::{AuthorizationCode, EmptyExtraTokenFields, StandardTokenResponse};

impl EsiClient {
    /// Retrieves a token from EVE Online's OAuth2 API.
    ///
    /// This method uses the configured EsiClient to retrieve a token from EVE Online's
    /// OAuth2 API using the provided authorization code. This will contain both your
    /// access token and refresh token.
    ///
    /// # Example
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use eve_esi::EsiClient;
    ///     use oauth2::TokenResponse;
    ///
    ///     // You can get the authorization code as a query parameter in your callback API route
    ///     // when a user is redirected back to your application after authorization.
    ///     let authorization_code = "authorization_code";
    ///
    ///     let esi_client = EsiClient::builder()
    ///         .user_agent("MyApp/1.0 (contact@example.com)")
    ///         .client_id("client_id")
    ///         .client_secret("client_secret")
    ///         .callback_url("http://localhost:8080/callback")
    ///         .build()
    ///         .expect("Failed to build EsiClient");
    ///
    ///     let token = esi_client.get_token(authorization_code).await.expect("Failed to get token");
    ///     let access_token = token.access_token();
    ///     let refresh_token = token.refresh_token();
    /// }
    /// ```
    ///
    /// See [SSO Example](https://github.com/hyziri/eve_esi/blob/dev/examples/sso.rs) for a more complete example.
    pub async fn get_token(
        &self,
        code: &str,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, EsiError> {
        let client = if let Some(ref client) = self.oauth_client {
            client
        } else {
            return Err(EsiError::OAuthError(OAuthError::OAuth2NotConfigured));
        };

        match client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&self.reqwest_client)
            .await
        {
            Ok(token) => Ok(token),
            Err(err) => Err(EsiError::OAuthError(OAuthError::TokenError(err))),
        }
    }
}
