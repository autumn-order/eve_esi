static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::test]
async fn get_character() {
    let mut mock_server = mockito::Server::new_async().await;

    let mock_server_url = mock_server.url();

    let expected_character = eve_esi::model::character::Character {
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

    let mut esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    esi_client.esi_url = mock_server_url.to_string();

    let character = esi_client
        .character()
        .get_character(2114794365)
        .await
        .unwrap();

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

    let mut esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    esi_client.esi_url = mock_server_url.to_string();

    let character = esi_client.character().get_character(2114794365).await;

    mock.assert();

    assert!(character.is_err());
}
