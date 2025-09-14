//! # EVE ESI Request Utilities
//!
//! Provides utility methods for making requests to EVE Online's ESI. These
//! methods are used internally by the [`crate::endpoints`] to make requests.
//!
//! Despite the use case intended primarily to be internal, these functions are exported publicly
//! to allow for using the ESI client to make requests to custom ESI routes. This is useful
//! for when this crate hasn't implemented an ESI route yet but you still wish to use the client
//! to make requests to the route.
//!
//! # Methods
//!
//! - [`EsiApi::new]: Creates a new instance of [`EsiApi`]
//! - [`EsiApi::get_from_public_esi`]: Makes an unauthenticated GET request to the ESI API.
//! - [`EsiApi::post_to_public_esi`]: Makes an unauthenticated POST request to the ESI API.
//!
//! # Usage
//!
//! ```no_run
//! use serde::{Serialize, Deserialize};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup a basic Client
//!     let esi_client = eve_esi::Client::builder()
//!         .user_agent("MyApp/1.0 (contact@example.com; +https://github.com/your/repository)")
//!         .build()
//!         .expect("Failed to build ESI Client");
//!
//!     // Define the struct to deserialize the ESI response to
//!     #[derive(Serialize, Deserialize)]
//!     pub struct CharacterAffiliation {
//!         pub alliance_id: Option<i64>,
//!         pub character_id: i64,
//!         pub corporation_id: i64,
//!         pub faction_id: Option<i64>,
//!     };
//!
//!     // Define the URL to make the request to
//!     let url = "https://esi.evetech.net/characters/affiliation/";
//!
//!     // Make the request with the earlier defined struct
//!     // - The first type, `<Vec<CharacterAffiliation>`, represents the response body to deserialize
//!     // - The second type, `Vec<i64>`, represents the request body to serialize
//!     let character_ids = vec![2114794365];
//!
//!     let affiliations = esi_client
//!         .esi()
//!         .post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i64>>(&url, &character_ids)
//!         .await;
//! }
//! ```

use serde::{de::DeserializeOwned, Serialize};

use crate::{model::oauth2::EveJwtClaims, Client, Error, OAuthError};

/// Provides utility methods for making requests EVE Online's ESI endpoints
///
/// See the [module-level documentation](self) for an overview, methods, & usage example.
pub struct EsiApi<'a> {
    pub(crate) client: &'a Client,
}

impl Client {
    /// Access to utility functions to make ESI requests
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    pub fn esi(&self) -> self::EsiApi<'_> {
        self::EsiApi::new(self)
    }
}

impl<'a> EsiApi<'a> {
    /// Creates a new instance of [`EsiApi`]
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `client` (&'a [`Client`]) used for making HTTP requests to EVE Online's ESI & OAuth2
    ///   endpoints and providing the JWT key caching & refresh handling used to validate tokens.
    ///
    /// # Returns
    /// - `Self`: A new instance of [`EsiApi`].
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Makes an unauthenticated GET request to the ESI API.
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]): The ESI API endpoint URL to request.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`
    pub async fn get_from_public_esi<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, reqwest::Error> {
        let reqwest_client = &self.client.inner.reqwest_client;

        let req = reqwest_client.get(url).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }

    /// Makes an unauthenticated POST request to the ESI API.
    ///
    /// See the [module-level documentation](self) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]):  The ESI API endpoint URL to request.
    /// - `data` ([`Serialize`]): The data to send in the request body.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`.
    /// - `U` - The type of data to send, which must implement `Serialize`.
    pub async fn post_to_public_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
        &self,
        url: &str,
        data: &U,
    ) -> Result<T, reqwest::Error> {
        let reqwest_client = &self.client.inner.reqwest_client;

        let req = reqwest_client.post(url).json(data).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }

    /// Makes an authenticated GET request to the ESI API.
    ///
    /// You can use the [`crate::ScopeBuilder`] with this method, calling [`crate::ScopeBuilder::build`] will
    /// convert it into a `Vec<String>` as required by this method's arguments.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]): The ESI API endpoint URL to request.
    /// - `access_token` (`&str`): Access token in &str format for making requests to authenticated ESI routes,
    ///   see [`crate::oauth2`] module docs for how to obtain an access token.
    /// - `scopes` (`Vec<String>`): Vec of strings representing the required scopes for an authenticated ESI route.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`
    pub async fn get_from_authenticated_esi<T: DeserializeOwned>(
        &self,
        url: &str,
        access_token: &str,
        scopes: Vec<String>,
    ) -> Result<T, Error> {
        if self.client.inner.esi_validate_token_before_request {
            // Ensure provided access token is valid
            let message = "Validating token prior to expiration & scope checks";
            log::debug!("{}", message);

            let claims = self
                .client
                .oauth2()
                .validate_token(access_token.to_string())
                .await?;

            // Check token claims to ensure token is not expired and it has required scopes
            self.check_token_claims(claims, scopes)?;

            let message = "Access token passed validation, expiration, and scope checks successfully prior to authenticated ESI request.";
            log::debug!("{}", message);
        };

        // Make the request
        let reqwest_client = &self.client.inner.reqwest_client;

        let bearer = format!("Bearer {}", access_token);

        let req = reqwest_client
            .get(url)
            .header("Authorization", bearer)
            .send()
            .await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }

    /// Utility function for authenticated routes to ensure access token is valid, has expected scopes, and is not expired
    ///
    /// You can use the [`crate::ScopeBuilder`] with this method, calling [`crate::ScopeBuilder::build`] will
    /// convert it into a Vec<String> as required by this method's arguments.
    ///
    /// # Arguments
    /// - `claims` ([`EveJwtClaims`]): Access token in &str format for making requests to authenticated ESI routes,
    ///   see [`crate::oauth2`] module docs for how to obtain an access token.
    /// - `scopes` (`Vec<String>`): Vec of strings representing the required scopes for an authenticated ESI route.
    ///
    /// # Returns
    /// - [`Error`] if either the token validation was unsuccessful, the token is expired, or the token is missing
    ///   required scopes.
    fn check_token_claims(&self, claims: EveJwtClaims, scopes: Vec<String>) -> Result<(), Error> {
        // Ensure token is not expired
        if claims.is_expired() {
            let error = OAuthError::AccessTokenExpired();

            let message = format!("Failed to make request to authenticated ESI route due to token being expired: {:?}",
            error);
            log::error!("{}", message);

            return Err(Error::OAuthError(error));
        } else {
            let message = "Checked access token for expiration prior to authenticated ESI request, token is not expired.";

            log::trace!("{}", message);
        }

        // Ensure token has required scopes
        if !claims.has_scopes(&scopes) {
            let error = OAuthError::AccessTokenMissingScopes(scopes);

            let message = format!(
                "Failed to make request to authenticated ESI route due to missing required scopes: {:?}", error

            );
            log::error!("{}", message);

            return Err(Error::OAuthError(error));
        } else {
            let message = format!("Checked access token for required scopes prior to authenticated ESI request, all required scopes are present: {:?}", scopes);
            log::trace!("{}", message);
        }

        Ok(())
    }
}

#[cfg(test)]
mod check_token_claims_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{model::oauth2::EveJwtClaims, tests::setup, Error, OAuthError};

    /// No errors should occur when checking token claims
    #[tokio::test]
    async fn test_check_token_claims_success() {
        // Setup basic ESI client for testing
        let (esi_client, _) = setup().await;

        // Create mock claims
        let required_scopes = vec!["publicData".to_string()];

        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = required_scopes.clone();

        // Check token claims
        let result = esi_client
            .esi()
            .check_token_claims(mock_claims, required_scopes);

        // Assert result is not error
        assert!(
            result.is_ok(),
            "Expected result Ok, instead got error: {:?}",
            result
        )
    }

    /// An error should occur due to token being expired
    #[tokio::test]
    async fn test_check_token_claims_expiration_error() {
        // Setup basic ESI client for testing
        let (esi_client, _) = setup().await;

        // Create mock claims that are expired
        let required_scopes = vec!["publicData".to_string()];

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.exp = now - 60;
        mock_claims.iat = now - 960;
        mock_claims.scp = required_scopes.clone();

        // Check token claims
        let result = esi_client
            .esi()
            .check_token_claims(mock_claims, required_scopes);

        // Assert result is error
        assert!(
            result.is_err(),
            "Expected error, instead got ok: {:?}",
            result
        );

        // Assert error is of type OAuthError::AccessTokenExpired
        assert!(
            matches!(
                result,
                Err(Error::OAuthError(OAuthError::AccessTokenExpired()))
            ),
            "Expected error of type OAuthError::AccessTokenExpired, instead got: {:?}",
            result
        )
    }

    /// An error should occur due to token being expired
    #[tokio::test]
    async fn test_check_token_claims_scope_error() {
        // Setup basic ESI client for testing
        let (esi_client, _) = setup().await;

        // Create mock claims missing the required "publicData" scope
        let required_scopes = vec!["publicData".to_string()];

        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = Vec::new();

        // Check token claims
        let result = esi_client
            .esi()
            .check_token_claims(mock_claims, required_scopes);

        // Assert result is error
        assert!(
            result.is_err(),
            "Expected error, instead got ok: {:?}",
            result
        );

        // Assert error is of type OAuthError::AccessTokenMissingScopes
        assert!(
            matches!(
                result,
                Err(Error::OAuthError(OAuthError::AccessTokenMissingScopes(_)))
            ),
            "Expected error of type OAuthError::AccessTokenMissingScopes, instead got: {:?}",
            result
        )
    }
}
