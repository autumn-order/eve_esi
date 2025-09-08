use serde::{de::DeserializeOwned, Serialize};

use crate::Client;

impl Client {
    /// Makes an unauthenticated GET request to the ESI API.
    ///
    /// # Arguments
    /// - `url` - The ESI API endpoint URL to request.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`
    pub(crate) async fn get_from_public_esi<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, reqwest::Error> {
        let req = self.reqwest_client.get(url).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }

    /// Makes an unauthenticated POST request to the ESI API.
    ///
    /// # Arguments
    /// - `url` - The ESI API endpoint URL to request.
    /// - `data` - The data to send in the request body.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or a reqwest error.
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`.
    /// - `U` - The type of data to send, which must implement `Serialize`.
    pub(crate) async fn post_to_public_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
        &self,
        url: &str,
        data: &U,
    ) -> Result<T, reqwest::Error> {
        let req = self.reqwest_client.post(url).json(data).send().await?;
        req.error_for_status_ref()?;
        let result: T = req.json().await?;
        Ok(result)
    }
}
