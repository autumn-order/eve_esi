use crate::model::corporation::Corporation;
use crate::EsiClient;

impl<'a> EsiClient<'a> {
    pub async fn get_corporation(
        &self,
        corporation_id: i32,
    ) -> Result<Corporation, reqwest::Error> {
        let url = format!(
            "https://esi.evetech.net/latest/corporations/{}/?datasource=tranquility",
            corporation_id
        );

        self.get_from_public_esi::<Corporation>(&url).await
    }
}
