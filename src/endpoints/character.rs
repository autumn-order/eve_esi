use crate::EsiClient;

use crate::model::character::{Character, CharacterAffiliation};

impl EsiClient {
    pub async fn get_character(&self, character_id: i32) -> Result<Character, reqwest::Error> {
        let url = format!(
            "{}/characters/{}/?datasource=tranquility",
            self.esi_url, character_id
        );

        self.get_from_public_esi::<Character>(&url).await
    }

    pub async fn get_character_affiliations(
        &self,
        character_ids: Vec<i32>,
    ) -> Result<Vec<CharacterAffiliation>, reqwest::Error> {
        let url = format!(
            "{}/characters/affiliation/?datasource=tranquility",
            self.esi_url
        );

        self.post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i32>>(&url, &character_ids)
            .await
    }
}
