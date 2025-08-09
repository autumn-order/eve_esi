static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Tests the successful retrieval of character information from a mock EVE ESI server.
///
/// # Test Setup
/// - Creates a mock server to simulate the ESI endpoint
/// - Configures a mock response with expected character information.
/// - Points the ESI client to the mock server URL
///
/// # Assertions
/// - Verifies that the returned character information matches the expected character information.
#[tokio::test]
async fn get_character_public_information() {
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
        .characters()
        .get_character_public_information(2114794365)
        .await
        .unwrap();

    mock.assert();

    assert_eq!(character, expected_character);
}

/// Tests receiving a 404 error when attempting to retrieve character information from a mock EVE ESI server.
///
/// # Test Setup
/// - Creates a mock server to simulate the ESI endpoint
/// - Configures a mock response with a 404 not found response
/// - Points the ESI client to the mock server URL
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the received result is a EsiError of the ReqwestError type with status code 404
#[tokio::test]
async fn get_character_public_information_not_found() {
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

    let result = esi_client
        .characters()
        .get_character_public_information(2114794365)
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
