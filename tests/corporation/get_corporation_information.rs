#[tokio::test]
async fn get_corporation() {
    let mut mock_server = mockito::Server::new_async().await;

    let mock_server_url = mock_server.url();

    let expected_corporation = eve_esi::model::corporation::Corporation {
        alliance_id: Some(99013534),
        ceo_id: 2114794365,
        creator_id: 2114794365,
        date_founded: Some("2024-10-07T21:43:09Z".parse().unwrap()),
        description: Some("".to_string()),
        home_station_id: Some(60003760),
        member_count: 21,
        name: "The Order of Autumn".to_string(),
        shares: Some(1000),
        tax_rate: 0.0,
        ticker: "F4LL.".to_string(),
        url: Some("https://autumn-order.com".to_string()),
        war_eligible: Some(true),
        faction_id: None,
    };

    let mock = mock_server.mock("GET", "/corporations/98785281/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"alliance_id": 99013534, "ceo_id": 2114794365, "creator_id": 2114794365, "date_founded": "2024-10-07T21:43:09Z", "description": "", "home_station_id": 60003760, "member_count": 21, "name": "The Order of Autumn", "shares": 1000, "tax_rate": 0, "ticker": "F4LL.", "url": "https://autumn-order.com", "war_eligible": true}"#)
            .create();

    let esi_client: eve_esi::EsiClient = eve_esi::EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .esi_url(&mock_server_url)
        .build()
        .expect("Failed to build EsiClient");

    let corporation = esi_client
        .corporation()
        .get_corporation_information(98785281)
        .await
        .unwrap();

    mock.assert();

    assert_eq!(corporation, expected_corporation);
}

#[tokio::test]
async fn get_corporation_not_found() {
    let mut mock_server = mockito::Server::new_async().await;

    let mock_server_url = mock_server.url();

    let mock = mock_server
        .mock("GET", "/corporations/99999999/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Corporation not found"}"#)
        .create();

    let esi_client: eve_esi::EsiClient = eve_esi::EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .esi_url(&mock_server_url)
        .build()
        .expect("Failed to build EsiClient");

    let result = esi_client
        .corporation()
        .get_corporation_information(99999999)
        .await;

    mock.assert();

    assert!(result.is_err());
}
