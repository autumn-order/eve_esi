use crate::{error::EsiError, model::alliance::Alliance, EsiClient};

impl EsiClient {
    pub async fn get_alliance(&self, alliance_id: i32) -> Result<Alliance, EsiError> {
        let url = format!(
            "{}/alliances/{}/?datasource=tranquility",
            self.esi_url, alliance_id
        );

        Ok(self.get_from_public_esi::<Alliance>(&url).await?)
    }
}
