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
    /// - `required_scopes` (`Vec<String>`): Vec of strings representing the required scopes for an authenticated ESI endpoint.
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
        required_scopes: Vec<String>,
    ) -> Result<T, Error> {
        self.validate_token_before_request(access_token, required_scopes)
            .await?;

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

    /// Utilty function which ensures token is valid, not expired, and has all required scopes
    async fn validate_token_before_request(
        &self,
        access_token: &str,
        scopes: Vec<String>,
    ) -> Result<(), Error> {
        if self.client.inner.esi_validate_token_before_request {
            let message = "Validating token prior to expiration & scope checks";
            log::debug!("{}", message);

            let claims = self
                .client
                .oauth2()
                .validate_token(access_token.to_string())
                .await?;

            check_token_expiration(&claims)?;

            check_token_scopes(&claims, scopes)?;

            let message = "Access token passed validation, expiration, and scope checks successfully prior to authenticated ESI request.";
            log::debug!("{}", message);
        };

        Ok(())
    }
}

/// Utility function for authenticated routes to ensure provided claims is not expired
fn check_token_expiration(claims: &EveJwtClaims) -> Result<(), Error> {
    if claims.is_expired() {
        let error = OAuthError::AccessTokenExpired();

        let message = format!(
            "Failed to make request to authenticated ESI route due to token being expired: {:?}",
            error
        );
        log::error!("{}", message);

        return Err(Error::OAuthError(error));
    }

    let message = "Checked access token for expiration prior to authenticated ESI request, token is not expired.";
    log::trace!("{}", message);

    Ok(())
}

/// Utility function for authenticated routes to ensure provided claims has required scopes
fn check_token_scopes(claims: &EveJwtClaims, scopes: Vec<String>) -> Result<(), Error> {
    if !claims.has_scopes(&scopes) {
        let error = OAuthError::AccessTokenMissingScopes(scopes);

        let message = format!(
            "Failed to make request to authenticated ESI route due to missing required scopes: {:?}", error

        );
        log::error!("{}", message);

        return Err(Error::OAuthError(error));
    }

    let message = format!("Checked access token for required scopes prior to authenticated ESI request, all required scopes are present: {:?}", scopes);
    log::trace!("{}", message);

    Ok(())
}

#[cfg(test)]
mod check_token_expiration_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::check_token_expiration;
    use crate::{model::oauth2::EveJwtClaims, Error, OAuthError};

    /// No errors due to token not being expired
    #[test]
    fn test_check_token_expiration_success() {
        let mock_claims = EveJwtClaims::mock();

        let result = check_token_expiration(&mock_claims);

        assert!(result.is_ok())
    }

    /// Error occurs due to token being expired
    #[test]
    fn test_check_token_expiration_error() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.exp = now - 60; // expired 1 minute ago
        mock_claims.iat = now - 960; // created 16 minutes ago

        let result = check_token_expiration(&mock_claims);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::AccessTokenExpired()))
        ))
    }
}

#[cfg(test)]
mod test_check_token_scopes {
    use super::check_token_scopes;
    use crate::{model::oauth2::EveJwtClaims, Error, OAuthError};

    /// No errors due to token having all required scopes
    #[test]
    fn test_check_token_claims_success() {
        let required_scopes = vec!["publicData".to_string()];

        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = required_scopes.clone();

        let result = check_token_scopes(&mock_claims, required_scopes);

        assert!(result.is_ok())
    }

    /// Error occurs due to token missing required scopes
    #[tokio::test]
    async fn test_check_token_claims_scope_error() {
        let required_scopes = vec!["publicData".to_string()];

        let mut mock_claims = EveJwtClaims::mock();
        mock_claims.scp = Vec::new();

        let result = check_token_scopes(&mock_claims, required_scopes);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::AccessTokenMissingScopes(_)))
        ))
    }
}
