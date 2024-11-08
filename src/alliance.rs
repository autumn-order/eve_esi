use crate::{model::alliance::Alliance, Client};

impl Client {
    pub async fn get_alliance(&self, alliance_id: i32) -> Result<Alliance, reqwest::Error> {
        let url = format!(
            "https://esi.evetech.net/latest/alliances/{}/?datasource=tranquility",
            alliance_id
        );

        self.get_from_public_esi::<Alliance>(&url).await
    }
}
