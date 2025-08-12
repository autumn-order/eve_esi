use crate::{error::EsiError, model::alliance::Alliance, EsiClient};

pub struct AllianceApi<'a> {
    client: &'a EsiClient,
}

impl<'a> AllianceApi<'a> {
    pub(crate) fn new(client: &'a EsiClient) -> Self {
        Self { client }
    }

    /// Retrieves information about a specific alliance from EVE Online's ESI API.
    ///
    /// This endpoint fetches public data about an alliance including name, ticker, date founded,
    /// executor corporation, and faction.
    ///
    /// # Arguments
    /// - `alliance_id` - The unique identifier for the alliance to look up
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Alliance`] - The alliance data if successfully retrieved
    /// - [`EsiError`] - An error if the request failed (e.g., alliance not found, network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceId).
    ///
    /// # Example
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let esi_client = eve_esi::EsiClient::builder()
    ///         .user_agent("MyApp/1.0 (contact@example.com)")
    ///         .build()
    ///         .expect("Failed to build EsiClient");
    ///
    ///     // Get information about The Autumn alliance (id: 99013534)
    ///     let alliance = esi_client.alliance().get_alliance_information(99013534).await.unwrap();
    ///     println!("Alliance name: {}", alliance.name);
    /// }
    /// ```
    pub async fn get_alliance_information(&self, alliance_id: i32) -> Result<Alliance, EsiError> {
        let url = format!("{}/alliances/{}/", self.client.esi_url, alliance_id);

        Ok(self.client.get_from_public_esi::<Alliance>(&url).await?)
    }
}
