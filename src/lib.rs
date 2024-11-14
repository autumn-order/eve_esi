pub mod model;

mod alliance;
mod character;
mod corporation;
mod esi;

use serde::{de::DeserializeOwned, Serialize};

pub struct Client {
    reqwest_client: reqwest::Client,
    pub esi_url: String,
}

impl Client {
    pub fn new(user_agent: &str) -> Self {
        Self {
            reqwest_client: reqwest::Client::builder()
                .user_agent(user_agent)
                .build()
                .unwrap(),
            esi_url: "https://esi.evetech.net/latest".to_string(),
        }
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
