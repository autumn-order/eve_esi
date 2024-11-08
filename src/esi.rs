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
