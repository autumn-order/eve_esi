use crate::Client;

use crate::model::character::{Character, CharacterAffiliation};

impl Client {
    pub async fn get_character(&self, character_id: i32) -> Result<Character, reqwest::Error> {
        let url = format!(
            "https://esi.evetech.net/latest/characters/{}/?datasource=tranquility",
            character_id
        );

        self.get_from_public_esi::<Character>(&url).await
    }

    pub async fn get_character_affiliations(
        &self,
        character_ids: Vec<i32>,
    ) -> Result<Vec<CharacterAffiliation>, reqwest::Error> {
        let url = "https://esi.evetech.net/latest/characters/affiliation/?datasource=tranquility";

        self.post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i32>>(url, &character_ids)
            .await
    }
}
