use serde::{de::DeserializeOwned, Serialize};

pub async fn get_from_public_esi<T: DeserializeOwned>(
    reqwest_client: &reqwest::Client,
    url: &str,
) -> Result<T, reqwest::Error> {
    let req = reqwest_client.get(url).send().await?;

    req.error_for_status_ref()?;

    let result: T = req.json().await?;

    Ok(result)
}

pub async fn post_to_public_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
    reqwest_client: &reqwest::Client,
    url: &str,
    data: &U,
) -> Result<T, reqwest::Error> {
    let req = reqwest_client.post(url).json(data).send().await?;

    req.error_for_status_ref()?;

    let result: T = req.json().await?;

    Ok(result)
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

        let client = reqwest::Client::new();
        let url = &format!("{}/test", mock_server_url);

        let result: TestResponse = super::get_from_public_esi(&client, url).await.unwrap();

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

        let client = reqwest::Client::new();
        let url = &format!("{}/test", mock_server_url);
        let data = json!({ "key": "value" });

        let result: TestResponse = super::post_to_public_esi(&client, url, &data)
            .await
            .unwrap();

        mock.assert();

        assert_eq!(
            result,
            TestResponse {
                message: "Hello, world!".to_string()
            }
        );
    }
}
