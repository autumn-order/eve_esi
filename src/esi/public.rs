//! # EVE ESI Public Request Methods
//!
//! Provides the underlying utility methods used to make requests to
//! ESI routes.
//!
//! See the [module-level documentation](super) for an overview, methods, & usage example.
//!
//! ## Methods
//! - [`EsiApi::get_from_public_esi`]: Makes an unauthenticated GET request to the ESI API.
//! - [`EsiApi::post_to_public_esi`]: Makes an unauthenticated POST request to the ESI API.

use serde::{de::DeserializeOwned, Serialize};

use super::EsiApi;
use crate::Error;

impl<'a> EsiApi<'a> {
    /// Makes an unauthenticated GET request to the ESI API.
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
    ///
    /// # Arguments
    /// - `url` ([`DeserializeOwned`]): The ESI API endpoint URL to request.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`
    pub async fn get_from_public_esi<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let reqwest_client = &self.client.inner.reqwest_client;

        let req = reqwest_client.get(url).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }

    /// Makes an unauthenticated POST request to the ESI API.
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
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
    ) -> Result<T, Error> {
        let reqwest_client = &self.client.inner.reqwest_client;

        let req = reqwest_client.post(url).json(data).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }
}
