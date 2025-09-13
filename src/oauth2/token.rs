//! # EVE Online OAuth2 Tokens
//!
//! Methods for fetching, refreshing, & validating tokens retrieved from EVE Online's OAuth2 API.
//!
//! ## Methods
//! - [OAuth2Api::get_token]: Retrieves a token from EVE Online's OAuth2 API
//! - [OAuth2Api::get_token_refresh]: Retrieves a new token using a refresh token
//! - [OAuth2Api::validate_token]: Validates token retrieved via the [`OAuth2Api::get_token`] method
//! - [OAuth2Api::check_token_expiration]: Checks if the provided access token is expired
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/docs/services/sso/>
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
//!     let refresh_token = token.refresh_token().unwrap();
//!
//!     // Validate the token
//!     let claims = esi_client
//!         .oauth2()
//!         .validate_token(access_token.secret().to_string())
//!         .await
//!         .expect("Failed to validate token");
//!
//!     // Extract character ID
//!     let id_str = claims.sub.split(':').collect::<Vec<&str>>()[2];
//!     let character_id: i32 = id_str.parse().expect("Failed to parse id to i32");
//!
//!     // Refresh the token
//!     let new_token = esi_client
//!         .oauth2()
//!         .get_token_refresh(refresh_token.secret().to_string())
//!         .await
//!         .expect("Failed to get refresh token");
//! }
//! ```

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, Validation};
use log::{debug, error, info, trace};
use oauth2::basic::BasicTokenType;
use oauth2::{
    AccessToken, AuthorizationCode, EmptyExtraTokenFields, RefreshToken, StandardTokenResponse,
};

use crate::error::{Error, OAuthError};
use crate::model::oauth2::{EveJwtClaims, EveJwtKey, EveJwtKeys};
use crate::oauth2::client::OAuth2Client;
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
    /// # Usage
    /// After successful usage of [Self::get_token], you can use these methods on the resulting token:
    ///
    /// - Access token: `token.access_token()`
    /// - Refresh token: `token.refresh_token()`
    ///
    /// The access token expires after 15 minutes, you can use [Self::get_token_refresh]
    /// to get a new token.
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
        let oauth_client = get_oauth_client(self.client)?;

        // Attempt to fetch token
        let message = "Attempting to fetch JWT token using provided authorization code";
        debug!("{}", message);

        match oauth_client
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

    /// Retrieves a new token using a refresh token
    ///
    /// Uses the configured client to fetch a fresh token using a provided refresh_token.
    /// This will allow for getting a new access token & refresh token which you should replace your old
    /// tokens with.
    ///
    /// This is similar to [`Self::get_token`], the only difference is you are requesting a token using a
    /// refresh token rather than an authorization code.
    ///
    /// For an overview & usage, see the [module-level documentation](super)
    ///
    /// # Documentation
    /// See <https://developers.eveonline.com/docs/services/sso/>
    ///
    /// # Arguments
    /// - `refresh_token` ([`String`]): A string representing a refresh token returned from the
    ///   [`Self::get_token`] method. You can get the refresh token from the token with the
    ///   `token.refresh_token()` method if you haven't yet converted it to a string for database
    ///   storage.
    ///
    /// # Errors
    /// - [`Error`]: If OAuth2 is not configured for the ESI client, the provided refresh_token
    ///   is invalid, or there is an issue fetching the JWT token from EVE Online's OAuth2 API.
    pub async fn get_token_refresh(
        &self,
        refresh_token: String,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Error> {
        let oauth_client = get_oauth_client(self.client)?;

        // Convert refresh_token string to RefreshToken
        let refresh_token = RefreshToken::new(refresh_token);

        // Attempt to refresh token
        let message = "Attempting to refresh JWT token using provided refresh token";
        debug!("{}", message);

        match oauth_client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.client.inner.reqwest_client)
            .await
        {
            Ok(token) => {
                debug!("{}", "JWT Token refreshed successfully");

                Ok(token)
            }
            Err(err) => {
                let message = format!("Error refreshing JWT token token: {:#?}", err);
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
                    let message = format!("Failed to validate JWT token due to error: {:#?}", &err);

                    debug!("{}", message);

                    Err(err)
                }
            }
        }
    }

    /// Checks if the provided access token is expired
    ///
    /// Use this method before fetching data from an authenticated ESI route to ensure your access
    /// token is not expired. This method uses the [`Self::validate_token`] method internally to access
    /// the claims and ensure the claims have not been modified.
    ///
    /// # Arguments
    /// - `access_token`: An access token in string format. If you haven't converted the
    ///   token to string yet, you can do so with `token.access_token().secret().to_string()`.
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - [`bool`]: A bool indicating whether or not the token is expired
    /// - [`Error`]: An error if the token validation used to retrieve the expiration from claims
    ///   fails. This generally happens if the access token provided is not a valid access
    ///   token or there is an issue fetching the JWT keys to validate the token.
    pub async fn check_token_expiration(&self, access_token: &str) -> Result<bool, Error> {
        let message =
            "Checking token expiration, attempting to validate token prior to expiration check.";
        log::debug!("{}", message);

        // Validate token to get claims
        let access_token = AccessToken::new(access_token.to_string());

        let claims = match self.validate_token(access_token.secret().to_string()).await {
            Ok(claims) => claims,
            Err(err) => {
                // Trace because the validate_token method already logs an error for this
                let message = format!(
                    "Failed to validate token for expiration check due to error: {}",
                    err
                );
                log::trace!("{}", message);

                return Err(err);
            }
        };

        // Logging: Get character ID for debug logging
        let id_str = claims.sub.split(':').collect::<Vec<&str>>()[2];
        let character_id: i64 = id_str.parse().expect("Failed to parse character id to i64");

        // Trace because validate_token already logs info for this
        let message = format!(
            "Successfully validated token for character ID {} prior to token expiration check",
            character_id
        );
        log::trace!("{}", message);

        // Check token expiration
        let expiration_secs = Duration::from_secs(claims.exp as u64);
        let expiration = UNIX_EPOCH + expiration_secs;

        if SystemTime::now() < expiration {
            let message = format!(
                "Checked token for expiration, token for character ID {} is not yet expired, expiration in {}s",
                character_id,
                expiration_secs.as_secs()
            );
            log::debug!("{}", message);

            // Return false, token is not yet expired
            return Ok(false);
        }

        let message = format!(
            "Checked token for expiration, token for character ID {} is expired",
            character_id
        );
        log::debug!("{}", message);

        // Return true, token is expired
        Ok(true)
    }
}

/// Utility function to validate that the scopes in the provided token claims matches those expected
///
/// # Arguments
/// - `expected_scopes` (Vec<String>): An array of scope strings validated against the
///   claims.scp field to ensure it contains all expected scopes.
/// - `claims` ([`EveJwtClaims`]): The claims of a token obtained via the [`Self::validate_token`]
///   method to be checked for the expected scopes.
///
/// # Returns
/// - `bool`: Bool indicating whether or not all expected scopes are present.
pub fn check_token_scopes(expected_scopes: Vec<String>, claims: EveJwtClaims) -> bool {
    // Logging: Get character ID for debug logging
    let id_str = claims.sub.split(':').collect::<Vec<&str>>()[2];
    let character_id: i64 = id_str.parse().expect("Failed to parse character id to i64");

    for expected_scope in &expected_scopes {
        if !claims.scp.iter().any(|scope| scope == expected_scope) {
            // One of the expected scopes is missing
            let message = format!(
                "Token for character ID {} is missing scope: {}",
                character_id, expected_scope
            );
            log::debug!("{}", message);

            return false;
        }
    }

    // All expected scopes were found
    let message = format!(
        "Token for character ID {} has all expected scopes",
        character_id
    );
    debug!("{}", message);

    true
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
    validation.set_issuer(&client.inner.jwt_issuers);

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

/// Utility function to retrieve OAuth2 client or return an error
fn get_oauth_client(client: &Client) -> Result<&OAuth2Client, Error> {
    // Attempt to retrieve OAuth2 client from ESI client
    trace!("{}", "Attempting to retrieve OAuth2 client from ESI client");

    match client.inner.oauth2_client {
        Some(ref client) => {
            trace!("{}", "Found OAuth2 client on ESI client");

            Ok(client)
        }
        None => {
            error!("{}", Error::OAuthError(OAuthError::OAuth2NotConfigured));

            // No OAuth2 client was found due to not being configured
            Err(Error::OAuthError(OAuthError::OAuth2NotConfigured))
        }
    }
}

#[cfg(test)]
mod check_token_scopes_tests {
    use crate::{model::oauth2::EveJwtClaims, oauth2::token::check_token_scopes};

    /// Test that function returns true since all scopes are present
    ///
    /// # Test Setup
    /// - Create mock token claims with expected scopes
    ///
    /// # Assert
    /// - Assert result is true
    #[test]
    fn test_check_token_scopes_true() {
        // Create mock token claims with expected scopes
        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = vec!["publicData".to_string()];

        // Test function
        let expected_scopes = vec!["publicData".to_string()];
        let result = check_token_scopes(expected_scopes, mock_claims);

        // Assert result is true
        assert_eq!(result, true);
    }

    /// Test that function returns false due to missing scopes
    ///
    /// # Test Setup
    /// - Create mock token claims with expected scopes
    ///
    /// # Assert
    /// - Assert result is false
    #[test]
    fn test_check_token_scopes_false() {
        // Create mock token claims missing expected scopes
        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = vec!["".to_string()];

        // Test function
        let expected_scopes = vec!["publicData".to_string()];
        let result = check_token_scopes(expected_scopes, mock_claims);

        // Assert result is false
        assert_eq!(result, false);
    }
}
