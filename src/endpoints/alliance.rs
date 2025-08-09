use crate::{model::alliance::Alliance, EsiClient};

impl EsiClient {
    pub async fn get_alliance(&self, alliance_id: i32) -> Result<Alliance, reqwest::Error> {
        let url = format!(
            "{}/alliances/{}/?datasource=tranquility",
            self.esi_url, alliance_id
        );

        self.get_from_public_esi::<Alliance>(&url).await
    }
}
