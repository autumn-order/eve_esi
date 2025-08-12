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
    ///     let esi_client = eve_esi::EsiClient::builder()
    ///         .user_agent("MyApp/1.0 (contact@example.com)")
    ///         .build()
    ///         .expect("Failed to build EsiClient");
    ///
    ///     // Get information about the character Hyziri (id: 2114794365)
    ///     let character = esi_client.characters().get_character_public_information(2114794365).await.unwrap();
    ///     println!("Character name: {}", character.name);
    /// }
    pub async fn get_character_public_information(
        &self,
        character_id: i32,
    ) -> Result<Character, EsiError> {
        let url = format!("{}/characters/{}/", self.client.esi_url, character_id);

        Ok(self.client.get_from_public_esi::<Character>(&url).await?)
    }

    /// Retrieve affiliations for a list of characters.
    ///
    /// This endpoint returns a list of affiliations for the requested characters.
    /// Each affiliation includes the character's corporation, alliance, and faction IDs.
    ///
    /// # Arguments
    /// - `character_ids` - A list of character IDs to retrieve affiliations for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Vec<CharacterAffiliation>`] - The affiliations for the characters if successfully retrieved
    /// - [`EsiError`] - An error if the request failed (network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/PostCharactersAffiliation).
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
    ///     // Get affiliations for characters with IDs 2114794365 and 2117053828
    ///     let affiliations = esi_client.character().character_affiliation(vec![2114794365, 2117053828]).await.unwrap();
    ///     for affiliation in affiliations {
    ///         let alliance_id = if let Some(alliance_id) = affiliation.alliance_id {
    ///             alliance_id.to_string()
    ///         } else {
    ///             "None".to_string()
    ///         };
    ///
    ///         println!("Character ID: {}, Alliance ID: {}, Corporation ID: {}", affiliation.character_id, alliance_id, affiliation.corporation_id);
    ///     }
    /// }
    pub async fn character_affiliation(
        &self,
        character_ids: Vec<i32>,
    ) -> Result<Vec<CharacterAffiliation>, EsiError> {
        let url = format!("{}/characters/affiliation/", self.client.esi_url);

        Ok(self
            .client
            .post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i32>>(&url, &character_ids)
            .await?)
    }
}
