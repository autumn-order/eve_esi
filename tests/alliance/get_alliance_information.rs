static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Tests the successful retrieval of alliance information from a mock EVE ESI server.
///
/// # Test Setup
/// - Creates a mock server to simulate the ESI API
/// - Configures a mock response with expected alliance data
/// - Points the ESI client to the mock server URL
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the retrieved alliance information matches the expected data
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

    let alliance: eve_esi::model::alliance::Alliance = esi_client
        .alliances()
        .get_alliance_information(99013534)
        .await
        .unwrap();

    mock.assert();

    assert_eq!(alliance, expected_alliance);
}

/// Tests the successful retrieval of alliance information from a mock EVE ESI server.
///
/// # Test Setup
/// - Creates a mock server to simulate the ESI API
/// - Configures a mock response with a 404 not found response
/// - Points the ESI client to the mock server URL
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the received result is a EsiError of the ReqwestError type with status code 404
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

    let result = esi_client
        .alliances()
        .get_alliance_information(99999999)
        .await;

    mock.assert();

    match result {
        Ok(_) => panic!("Expected Err"),
        Err(eve_esi::error::EsiError::ReqwestError(reqwest_error)) => {
            assert!(reqwest_error.status().is_some());
            assert_eq!(
                reqwest_error.status().unwrap(),
                reqwest::StatusCode::NOT_FOUND
            );
        }
        Err(_) => panic!("Expected EsiError::ReqwestError with status code 404"),
    }
}
