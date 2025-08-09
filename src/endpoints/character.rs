use crate::error::EsiError;
use crate::EsiClient;

use crate::model::character::{Character, CharacterAffiliation};

pub struct CharacterApi<'a> {
    client: &'a EsiClient,
}

impl<'a> CharacterApi<'a> {
    pub fn new(client: &'a EsiClient) -> Self {
        Self { client }
    }

    /// Retrieves information about a specific character from EVE Online's ESI API.
    ///
    /// This endpoint fetches character information based on the provided character ID.
    /// The endpoint returns data such as the character's name, corporation, alliance_id,
    /// and other relevant information.
    ///
    /// # Arguments
    /// - `character_id` - The ID of the character to retrieve information for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Character`] - The character data if successfully retrieved
    /// - [`EsiError`] - An error if the request failed (e.g., character not found, network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterId).
    ///
    /// # Example
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let esi_client = eve_esi::EsiClient::new("MyApp/1.0 (user@example.com)");
    ///
    ///     // Get information about the character Hyziri (id: 2114794365)
    ///     let character = esi_client.character().get_character(2114794365).await.unwrap();
    ///     println!("Character name: {}", character.name);
    /// }
    pub async fn get_character(&self, character_id: i32) -> Result<Character, EsiError> {
        let url = format!(
            "{}/characters/{}/?datasource=tranquility",
            self.client.esi_url, character_id
        );

        Ok(self.client.get_from_public_esi::<Character>(&url).await?)
    }

    pub async fn get_character_affiliations(
        &self,
        character_ids: Vec<i32>,
    ) -> Result<Vec<CharacterAffiliation>, EsiError> {
        let url = format!(
            "{}/characters/affiliation/?datasource=tranquility",
            self.client.esi_url
        );

        Ok(self
            .client
            .post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i32>>(&url, &character_ids)
            .await?)
    }
}
