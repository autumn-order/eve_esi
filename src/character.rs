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

#[cfg(test)]
mod tests {
    use super::*;

    static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    #[tokio::test]
    async fn get_character() {
        let mut mock_server = mockito::Server::new_async().await;

        let mock_server_url = mock_server.url();

        let expected_character = Character {
            alliance_id: Some(99013534),
            birthday: "2018-12-20T16:11:54Z".parse().unwrap(),
            bloodline_id: 7,
            corporation_id: 98785281,
            description: Some("description".to_string()),
            faction_id: None,
            gender: "male".to_string(),
            name: "Hyziri".to_string(),
            race_id: 8,
            security_status: Some(-0.10037364300000001),
            title: Some("Serial CEO".to_string()),
        };

        let mock = mock_server
            .mock("GET", "/characters/2114794365/?datasource=tranquility")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "alliance_id": 99013534,
                    "birthday": "2018-12-20T16:11:54Z",
                    "bloodline_id": 7,
                    "corporation_id": 98785281,
                    "description": "description",
                    "gender": "male",
                    "name": "Hyziri",
                    "race_id": 8,
                    "security_status": -0.10037364300000001,
                    "title": "Serial CEO"
                }"#,
            )
            .create();

        let mut esi_client: crate::EsiClient = crate::EsiClient::new(USER_AGENT);

        esi_client.esi_url = mock_server_url.to_string();

        let character = esi_client.get_character(2114794365).await.unwrap();

        mock.assert();

        assert_eq!(character, expected_character);
    }

    #[tokio::test]
    async fn get_character_not_found() {
        let mut mock_server = mockito::Server::new_async().await;

        let mock_server_url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/characters/2114794365/?datasource=tranquility")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "Character not found"}"#)
            .create();

        let mut esi_client: crate::EsiClient = crate::EsiClient::new(USER_AGENT);

        esi_client.esi_url = mock_server_url.to_string();

        let character = esi_client.get_character(2114794365).await;

        mock.assert();

        assert!(character.is_err());
    }

    #[tokio::test]
    async fn get_character_affiliations() {
        let mut mock_server = mockito::Server::new_async().await;

        let mock_server_url = mock_server.url();

        let expected_character_affiliations = vec![
            CharacterAffiliation {
                character_id: 2114794365,
                corporation_id: 98785281,
                alliance_id: Some(99013534),
                faction_id: None,
            },
            CharacterAffiliation {
                character_id: 2117053828,
                corporation_id: 98785281,
                alliance_id: Some(99013534),
                faction_id: None,
            },
        ];

        let mock = mock_server
            .mock("POST", "/characters/affiliation/?datasource=tranquility")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{
                    "character_id": 2114794365,
                    "corporation_id": 98785281,
                    "alliance_id": 99013534,
                    "faction_id": null
                },
                {
                    "character_id": 2117053828,
                    "corporation_id": 98785281,
                    "alliance_id": 99013534,
                    "faction_id": null
                }]"#,
            )
            .create();

        let mut esi_client: crate::EsiClient = crate::EsiClient::new(USER_AGENT);

        esi_client.esi_url = mock_server_url.to_string();

        let character_affiliations = esi_client
            .get_character_affiliations(vec![2114794365, 2117053828])
            .await
            .unwrap();

        mock.assert();

        assert_eq!(character_affiliations, expected_character_affiliations);
    }
}
