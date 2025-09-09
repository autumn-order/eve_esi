//! # EVE Online OAuth2 Tokens
//!
//! Methods for retrieving & validating tokens retrieved from EVE Online's OAuth2 API.
//!
//! ## Methods
//! - [OAuth2Api::get_token]: Retrieves a token from EVE Online's OAuth2 API
//! - [OAuth2Api::validate_token]: Validates token retrieved via the [`OAuth2Api::get_token`] method
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
use log::{debug, error, info, trace};
use oauth2::basic::BasicTokenType;
use oauth2::{AuthorizationCode, EmptyExtraTokenFields, StandardTokenResponse};

use crate::error::{Error, OAuthError};
use crate::model::oauth2::{EveJwtClaims, EveJwtKey, EveJwtKeys};
use crate::oauth2::OAuth2Api;
use crate::Client;

impl<'a> OAuth2Api<'a> {
    /// Retrieves a token from EVE Online's OAuth2 API
    ///
    /// This method uses the configured Client to retrieve a token from EVE Online's
    /// OAuth2 API using the provided authorization code. This will contain both your
    /// access token and refresh token. The access token contains the character ID which you
    /// can access after validation. See [Self::validate_token] for token validation.
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
        // Attempt to retrieve OAuth2 client from ESI client

        trace!("{}", "Attempting to retrieve OAuth2 client from ESI client");

        let client = match &self.client.inner.oauth2_client {
            Some(client) => {
                trace!("{}", "Found OAuth2 client on ESI client");

                client
            }
            None => {
                error!("{}", Error::OAuthError(OAuthError::OAuth2NotConfigured));

                // No OAuth2 client was found due to not being configured
                return Err(Error::OAuthError(OAuthError::OAuth2NotConfigured));
            }
        };

        // Attempt to fetch token
        let message = "Attempting to fetch JWT token using provided authorization code";
        debug!("{}", message);

        match client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&self.client.inner.reqwest_client)
            .await
        {
            Ok(token) => {
                debug!("{}", "JWT Token fetched successfully");

                Ok(token)
            }
            Err(err) => {
                let message = format!("Error fetching token: {:#?}", err);
                error!("{}", message);

                Err(Error::OAuthError(OAuthError::RequestTokenError(err)))
            }
        }
    }

    /// Validates token retrieved via the [`Self::get_token`] method
    ///
    /// This will validate the token with an RS256 JWT key which will either be
    /// fetched EVE's OAuth2 API or retrieved from cache via the
    /// [`crate::oauth2::jwk::JwkApi::get_jwt_keys`] method.
    ///
    /// This function will make 2 attempts to validate a token, if the first attempt
    /// fails the JWT key cache will be cleared and a refresh attempt will be made.
    /// This is useful for when EVE Online rotates the JWT keys used to validate
    /// tokens and the keys need to be refetched.
    ///
    /// For a general overview on tokens & usage, see the [module-level documentation](super)
    ///
    /// # Documentation
    /// See <https://developers.eveonline.com/docs/services/sso/#validating-jwt-tokens>
    ///
    /// # Arguments
    /// - `token_secret` ([`String`]): The access token secret as a string. You can use
    ///   `token.access_token().secret().to_string()` on the token returned from [`Self::get_token`].
    ///
    /// # Errors
    /// - [`Error`]: If the retry attempt fails and there is an issue retrieving JWT keys from ESI Client's
    ///   cache or there is an issue validating the token.
    pub async fn validate_token(&self, token_secret: String) -> Result<EveJwtClaims, Error> {
        debug!("Attempting JWT token validation");

        // First attempt
        match attempt_validation(&self.client, &token_secret).await {
            Ok(claims) => Ok(claims),
            Err(err) => {
                // Clear the cache to trigger a JWT key refresh on next attempt
                let cache_cleared = self.client.inner.jwt_key_cache.clear_cache().await;

                // Second attempt (retry) if cache was successfully cleared
                if cache_cleared {
                    let message = format!(
                        "Making 2nd attempt to validate token due to previous error: {:#?}",
                        &err
                    );

                    debug!("{}", message);

                    attempt_validation(&self.client, &token_secret).await
                } else {
                    let message = format!(
                        "Making 2nd attempt to validate token due to previous error: {:#?}",
                        &err
                    );

                    debug!("{}", message);

                    Err(err)
                }
            }
        }
    }
}

/// Attempts to validate a token retrieved via the [`Self::get_token`] method
///
/// This is the internal utility method for token validation, see [`OAuth2Api::validate_token`]
/// for an overview.
///
/// # Arguments
/// - `client` (&[`Client`]): client used to make ESI & EVE OAuth2 requests and cache JWT keys
/// - `token_secret` ([`String`]): The access token secret as a string. You can use
///   `token.access_token().secret().to_string()` on the token returned from [`Self::get_token`].
///
/// # Errors
/// - [`Error`]: If there is an issue retrieving JWT keys from ESI Client's cache or there is an
///   issue validating the token.
async fn attempt_validation(client: &Client, token_secret: &str) -> Result<EveJwtClaims, Error> {
    // Get JWT keys to validate token
    trace!("Retrieving keys for validation from JWT key cache");

    let jwt_keys = client.oauth2().jwk().get_jwt_keys().await?;

    // Configure validation
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&[client.inner.jwt_audience.to_string()]);
    validation.set_issuer(&[client.inner.jwt_issuer.to_string()]);

    // Try to find an RS256 key
    trace!("Checking JWT key cache for RS256 key");

    if let Some(EveJwtKey::RS256 { ref n, ref e, .. }) = get_first_rs256_key(&jwt_keys) {
        // RS256 key was found, extract n (modulus) and e (exponent) components for the decoding key
        trace!("Creating a decoding key from RS256 key");

        let decoding_key = match DecodingKey::from_rsa_components(n, e) {
            Ok(key) => {
                trace!("Created decoding key from RS256 key successfully");

                key
            }
            Err(err) => {
                error!("Failed to decode RS256 key for token validation: {}", &err);

                return Err(Error::OAuthError(OAuthError::ValidateTokenError(err)));
            }
        };

        // Validate the token
        debug!("Validating token using RS256 decoding key");

        match jsonwebtoken::decode::<EveJwtClaims>(&token_secret, &decoding_key, &validation) {
            Ok(token_data) => {
                let id_str = token_data.claims.sub.split(':').collect::<Vec<&str>>()[2];
                let character_id: i32 = id_str.parse().expect("Failed to parse id to i32");

                let message = format!(
                    "Successfully validated JWT token for character ID: {}",
                    character_id
                );

                info!("{}", message);

                Ok(token_data.claims)
            }
            Err(err) => {
                let message = format!("Failed to validate token with RS256 key: {}", &err);

                error!("{}", message);

                Err(Error::OAuthError(OAuthError::ValidateTokenError(err)))
            }
        }
    } else {
        // No RS256 key was found
        let message: &str =
            "Failed to find RS256 key in JWT key cache when attempting to validate a JWT token.";

        error!("{}", message);

        Err(Error::OAuthError(OAuthError::NoValidKeyFound(
            message.to_string(),
        )))
    }
}

/// Get the first RS256 key (if any) from [`EveJwtKeys`]
fn get_first_rs256_key(jwt_keys: &EveJwtKeys) -> Option<&EveJwtKey> {
    jwt_keys
        .keys
        .iter()
        .find(|key| matches!(key, EveJwtKey::RS256 { .. }))
}
