static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::test]
async fn get_character_affiliations() {
    let mut mock_server = mockito::Server::new_async().await;

    let mock_server_url = mock_server.url();

    let expected_character_affiliations = vec![
        eve_esi::model::character::CharacterAffiliation {
            character_id: 2114794365,
            corporation_id: 98785281,
            alliance_id: Some(99013534),
            faction_id: None,
        },
        eve_esi::model::character::CharacterAffiliation {
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

    let mut esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    esi_client.esi_url = mock_server_url.to_string();

    let character_affiliations = esi_client
        .characters()
        .character_affiliation(vec![2114794365, 2117053828])
        .await
        .unwrap();

    mock.assert();

    assert_eq!(character_affiliations, expected_character_affiliations);
}
