use crate::error::EsiError;
use crate::model::corporation::Corporation;
use crate::EsiClient;

pub struct CorporationApi<'a> {
    client: &'a EsiClient,
}

impl<'a> CorporationApi<'a> {
    pub fn new(client: &'a EsiClient) -> Self {
        Self { client }
    }

    /// Retrieves information about a corporation from EVE Online's ESI API using the provided corporation ID.
    ///
    /// This endpoint fetches corporation information, returning data such as name, ticker, member count
    /// and other relevant information.
    ///
    /// # Arguments
    /// - `corporation_id` - The ID of the corporation to retrieve information for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Corporation`] - The corporation information if the request was successful.
    /// - [`EsiError`] - An error if the request failed (e.g. corporation not found, network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationId)
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
    ///     // Get information about the corporation The Order of Autumn (id: 98785281)
    ///     let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();
    ///     println!("Corporation name: {}", corporation.name);
    /// }
    /// ```
    pub async fn get_corporation_information(
        &self,
        corporation_id: i32,
    ) -> Result<Corporation, EsiError> {
        let url = format!("{}/corporations/{}/", self.client.esi_url, corporation_id);

        Ok(self.client.get_from_public_esi::<Corporation>(&url).await?)
    }
}
