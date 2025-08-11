use serde::{de::DeserializeOwned, Serialize};

use crate::EsiClient;

impl EsiClient {
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestResponse {
        message: String,
    }

    #[tokio::test]
    async fn get_from_public_esi() {
        let mut mock_server = mockito::Server::new_async().await;

        let mock_server_url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Hello, world!"}"#)
            .create();

        let esi_client = crate::EsiClient::new();
        let url = &format!("{}/test", mock_server_url);

        let result: TestResponse = esi_client.get_from_public_esi(url).await.unwrap();

        mock.assert();

        assert_eq!(
            result,
            TestResponse {
                message: "Hello, world!".to_string()
            }
        );
    }

    #[tokio::test]
    async fn post_to_public_esi() {
        let mut mock_server = mockito::Server::new_async().await;

        let mock_server_url = mock_server.url();

        let mock = mock_server
            .mock("POST", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Hello, world!"}"#)
            .create();

        let esi_client = crate::EsiClient::new();
        let url = &format!("{}/test", mock_server_url);
        let data = json!({ "key": "value" });

        let result: TestResponse = esi_client.post_to_public_esi(url, &data).await.unwrap();

        mock.assert();

        assert_eq!(
            result,
            TestResponse {
                message: "Hello, world!".to_string()
            }
        );
    }
}
