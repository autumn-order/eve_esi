pub mod model;

mod alliance;
mod character;
mod corporation;
mod esi;

use serde::{de::DeserializeOwned, Serialize};

pub struct Client {
    reqwest_client: reqwest::Client,
}

impl Client {
    pub fn new(reqwest_client: reqwest::Client) -> Self {
        Self { reqwest_client }
    }

    async fn get_from_public_esi<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, reqwest::Error> {
        esi::get_from_public_esi(&self.reqwest_client, url).await
    }

    async fn post_to_public_esi<T: DeserializeOwned, U: Serialize + ?Sized>(
        &self,
        url: &str,
        data: &U,
    ) -> Result<T, reqwest::Error> {
        esi::post_to_public_esi(&self.reqwest_client, url, data).await
    }
}
