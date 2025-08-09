use crate::model::corporation::Corporation;
use crate::EsiClient;

impl EsiClient {
    pub async fn get_corporation(
        &self,
        corporation_id: i32,
    ) -> Result<Corporation, reqwest::Error> {
        let url = format!(
            "{}/corporations/{}/?datasource=tranquility",
            self.esi_url, corporation_id
        );

        self.get_from_public_esi::<Corporation>(&url).await
    }
}
