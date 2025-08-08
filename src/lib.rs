pub mod model;

mod alliance;
mod character;
mod corporation;
mod error;
mod esi;
mod oauth2;

use serde::{de::DeserializeOwned, Serialize};

pub struct Client {
    reqwest_client: reqwest::Client,
    client_id: Option<String>,
    client_secret: Option<String>,
    pub esi_url: String,
    pub eve_auth_url: String,
    pub eve_auth_token_url: String,
}

impl Client {
    pub fn new(user_agent: &str) -> Self {
        Self {
            reqwest_client: reqwest::Client::builder()
                .user_agent(user_agent)
                .build()
                .unwrap(),
            client_id: None,
            client_secret: None,
            esi_url: "https://esi.evetech.net/latest".to_string(),
            eve_auth_url: "https://login.eveonline.com/v2/oauth/".to_string(),
            eve_auth_token_url: "https://login.eveonline.com/v2/oauth/token".to_string(),
        }
    }

    pub fn set_client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn set_client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
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
