static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::test]
async fn get_alliance() {
    let mut mock_server = mockito::Server::new_async().await;

    let mock_server_url = mock_server.url();

    let expected_alliance = eve_esi::model::alliance::Alliance {
        creator_corporation_id: 98784257,
        creator_id: 2114794365,
        faction_id: None,
        date_founded: "2024-09-25T06:25:58Z".parse().unwrap(),
        executor_corporation_id: Some(98787881),
        name: "Autumn.".to_string(),
        ticker: "AUTMN".to_string(),
    };

    let mock = mock_server.mock("GET", "/alliances/99013534/?datasource=tranquility")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"creator_corporation_id": 98784257, "creator_id": 2114794365, "date_founded": "2024-09-25T06:25:58Z", "executor_corporation_id": 98787881, "name": "Autumn.", "ticker": "AUTMN"}"#)
        .create();

    let mut esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    esi_client.esi_url = mock_server_url.to_string();

    let alliance: eve_esi::model::alliance::Alliance =
        esi_client.alliance().get_alliance(99013534).await.unwrap();

    mock.assert();

    assert_eq!(alliance, expected_alliance);
}

#[tokio::test]
async fn get_alliance_not_found() {
    let mut mock_server = mockito::Server::new_async().await;

    let mock_server_url = mock_server.url();

    let mock = mock_server
        .mock("GET", "/alliances/99999999/?datasource=tranquility")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Alliance not found"}"#)
        .create();

    let mut esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    esi_client.esi_url = mock_server_url.to_string();

    let result = esi_client.alliance().get_alliance(99999999).await;

    mock.assert();

    assert!(result.is_err());
}
