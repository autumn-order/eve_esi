//! # EVE ESI Authenticated Request Methods
//!
//! Provides the underlying utility methods used to make authenticated requests to
//! ESI endpoints.
//!
//! See the [module-level documentation](super) for an overview, methods, & usage example.
//!
//! ## Methods
//! - [`EsiApi::get_from_authenticated_esi`]: Makes an authenticated GET request to the ESI API using an access token.

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::EsiApi;
use crate::Error;

impl<'a> EsiApi<'a> {
    /// Makes an authenticated GET request to the ESI API using an access token.
    ///
    /// You can use the [`crate::ScopeBuilder`] with this method, calling [`crate::ScopeBuilder::build`] will
    /// convert it into a `Vec<String>` as required by this method's arguments.
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
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

    /// Makes an authenticated POST request to the ESI API using an access token.
    ///
    /// You can use the [`crate::ScopeBuilder`] with this method, calling [`crate::ScopeBuilder::build`] will
    /// convert it into a `Vec<String>` as required by this method's arguments.
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]):  The ESI API endpoint URL to request.
    /// - `data` ([`Serialize`]): The data to send in the request body.
    /// - `access_token` (`&str`): Access token in &str format for making requests to authenticated ESI routes,
    ///   see [`crate::oauth2`] module docs for how to obtain an access token.
    /// - `required_scopes` (`Vec<String>`): Vec of strings representing the required scopes for an authenticated ESI endpoint.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or an error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`.
    /// - `U` - The type of data to send, which must implement `Serialize`.
    pub async fn post_to_authenticated_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
        &self,
        url: &str,
        data: &U,
        access_token: &str,
        required_scopes: Vec<String>,
    ) -> Result<T, Error> {
        self.validate_token_before_request(access_token, required_scopes)
            .await?;

        let reqwest_client = &self.client.inner.reqwest_client;

        let bearer = format!("Bearer {}", access_token);

        let req = reqwest_client
            .post(url)
            .header("Authorization", bearer)
            .json(data)
            .send()
            .await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }

    /// Makes an authenticated PUT request to the ESI API using an access token.
    ///
    /// You can use the [`crate::ScopeBuilder`] with this method, calling [`crate::ScopeBuilder::build`] will
    /// convert it into a `Vec<String>` as required by this method's arguments.
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]):  The ESI API endpoint URL to request.
    /// - `data` ([`Serialize`]): The data to send in the request body.
    /// - `access_token` (`&str`): Access token in &str format for making requests to authenticated ESI routes,
    ///   see [`crate::oauth2`] module docs for how to obtain an access token.
    /// - `required_scopes` (`Vec<String>`): Vec of strings representing the required scopes for an authenticated ESI endpoint.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or an error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`.
    /// - `U` - The type of data to send, which must implement `Serialize`.
    pub async fn put_to_authenticated_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
        &self,
        url: &str,
        data: &U,
        access_token: &str,
        required_scopes: Vec<String>,
    ) -> Result<T, Error> {
        self.validate_token_before_request(access_token, required_scopes)
            .await?;

        let reqwest_client = &self.client.inner.reqwest_client;

        let bearer = format!("Bearer {}", access_token);

        let req = reqwest_client
            .put(url)
            .header("Authorization", bearer)
            .json(data)
            .send()
            .await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }
}
