//! # EVE Online OAuth2 Tokens
//!
//! Methods for retrieving & validating tokens retrieved from EVE Online's OAuth2 API.
//!
//! ## Methods
//! - [OAuth2Api::get_token]: Retrieves a token from EVE Online's OAuth2 API
//! - [OAuth2Api::validate_token]: Validates token retrieved via the [`Self::get_token`] method
//!
//! ## Documentation
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! ## Usage
//!
//! This demonstrates an example of a callback API route implemented in the Axum web framework.
//! See [SSO example](https://github.com/hyziri/eve_esi/blob/main/examples/sso.rs) for a more complete demonstration.
//!
//! ```no_run
//! use axum::extract::{Query, Extension};
//! use oauth2::TokenResponse;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct CallbackParams {
//!    state: String,
//!    code: String,
//! }
//!
//! async fn callback_route(
//!     Extension(esi_client): Extension<eve_esi::Client>,
//!     params: Query<CallbackParams>,
//! ) {
//!     ///Validate state to prevent CSRF...
//!
//!     // Fetch the token
//!     let token = esi_client
//!         .oauth2()
//!         .get_token(&params.0.code)
//!         .await
//!         .expect("Failed to get token");
//!
//!     let access_token = token.access_token();
//!     let refresh_token = token.refresh_token();
//!
//!     // Validate the token
//!     let claims = esi_client
//!         .oauth2()
//!         .validate_token(access_token.secret().to_string())
//!         .await
//!         .expect("Failed to validate token");
//!
//!
//!     // Extract character ID
//!     let id_str = claims.sub.split(':').collect::<Vec<&str>>()[2];
//!     let character_id: i32 = id_str.parse().expect("Failed to parse id to i32");
//! }
//! ```

use jsonwebtoken::{DecodingKey, Validation};
use oauth2::basic::BasicTokenType;
use oauth2::{AuthorizationCode, EmptyExtraTokenFields, StandardTokenResponse};

use crate::error::{Error, OAuthError};
use crate::model::oauth2::{EveJwtClaims, EveJwtKey, EveJwtKeys};
use crate::oauth2::OAuth2Api;

impl<'a> OAuth2Api<'a> {
    /// Retrieves a token from EVE Online's OAuth2 API
    ///
    /// This method uses the configured Client to retrieve a token from EVE Online's
    /// OAuth2 API using the provided authorization code. This will contain both your
    /// access token and refresh token. The access token contains the character ID which you
    /// can access after validation.
    ///
    /// See docs for [Self::validate_token] for token validation.
    ///
    /// For an overview & usage, see the [module-level documentation](super)
    ///
    /// # Documentation
    /// See <https://developers.eveonline.com/docs/services/sso/#authorization-code>
    ///
    /// # Arguments
    /// - `code` (&[`str`]): Authorization code returned in the callback API route query parameters
    ///   for your application.
    ///
    /// # Errors
    /// - [`Error`]: If OAuth2 is not configured for the ESI client or there is an issue fetching
    ///   the JWT token from EVE Online's OAuth2 API.
    pub async fn get_token(
        &self,
        code: &str,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Error> {
        let client = match &self.client.inner.oauth2_client {
            Some(client) => client,
            None => return Err(Error::OAuthError(OAuthError::OAuth2NotConfigured)),
        };

        match client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&self.client.inner.reqwest_client)
            .await
        {
            Ok(token) => Ok(token),
            Err(err) => Err(Error::OAuthError(OAuthError::RequestTokenError(err))),
        }
    }

    /// Validates token retrieved via the [`Self::get_token`] method
    ///
    /// This will validate the token with an RS256 JWT key which will either be
    /// fetched EVE's OAuth2 API or retrieved from cache via the
    /// [`crate::oauth2::jwk::JwkApi::get_jwt_keys`] method.
    ///
    /// For an overview & usage, see the [module-level documentation](super)
    ///
    /// # Documentation
    /// See <https://developers.eveonline.com/docs/services/sso/#validating-jwt-tokens>
    ///
    /// # Arguments
    /// - `token_secret` ([`String`]): The access token secret as a string. You can use
    ///   `token.access_token().secret().to_string()` on the token returned from [`Self::get_token`].
    ///
    /// # Errors
    /// - [`Error`]: If there is an issue retrieving JWT keys from ESI Client's cache or there is an
    ///   issue validating the token.
    pub async fn validate_token(&self, token_secret: String) -> Result<EveJwtClaims, Error> {
        // Get JWT keys to validate token
        let jwt_keys = self.jwk().get_jwt_keys().await?;

        // Configure validation
        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&[self.client.inner.jwt_audience.to_string()]);
        validation.set_issuer(&[self.client.inner.jwt_issuer.to_string()]);

        // Try to find an RS256 key
        if let Some(EveJwtKey::RS256 { ref n, ref e, .. }) = get_first_rs256_key(&jwt_keys) {
            // RS256 key was found, extract x and y components for the decoding key
            let decoding_key = match DecodingKey::from_rsa_components(n, e) {
                Ok(key) => key,
                Err(err) => return Err(Error::OAuthError(OAuthError::ValidateTokenError(err))),
            };

            // Validate the token
            match jsonwebtoken::decode::<EveJwtClaims>(&token_secret, &decoding_key, &validation) {
                Ok(token_data) => Ok(token_data.claims),
                Err(err) => Err(Error::OAuthError(OAuthError::ValidateTokenError(err))),
            }
        } else {
            // No RS256 key was found
            Err(Error::OAuthError(OAuthError::NoValidKeyFound(
                "Failed to find RS256 key in JWT key cache when attempting to validate a JWT token.".to_string(),
            )))
        }
    }
}

/// Get the first RS256 key (if any) from [`EveJwtKeys`]
fn get_first_rs256_key(jwt_keys: &EveJwtKeys) -> Option<&EveJwtKey> {
    jwt_keys
        .keys
        .iter()
        .find(|key| matches!(key, EveJwtKey::RS256 { .. }))
}
