static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Tests the successful retrieval of character affiliations from a mock EVE ESI server.
///
/// # Test Setup
/// - Creates a mock server to simulate the ESI endpoint
/// - Configures a mock response with expected character affiliation data for two characters
/// - Points the ESI client to the mock server URL
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that the retrieved character affiliation information matches the expected data
#[tokio::test]
async fn character_affiliation() {
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
        .mock("POST", "/characters/affiliation/")
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

/// Tests the successful retrieval of character affiliations from a mock EVE ESI server.
///
/// # Test Setup
/// - Creates a mock server to simulate the ESI endpoint
/// - Configures a mock response with a 400 bad request error with body "Invalid character ID"
/// - Points the ESI client to the mock server URL
///
/// # Assertions
/// - Verifies that a request has been made to the mock server
/// - Verifies that an error 400 Bad Request was received
#[tokio::test]
async fn character_affiliation_bad_request() {
    let mut mock_server = mockito::Server::new_async().await;

    let mock_server_url = mock_server.url();

    let mock = mock_server
        .mock("POST", "/characters/affiliation/")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "error": "Invalid character ID"
            }"#,
        )
        .create();

    let mut esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    esi_client.esi_url = mock_server_url.to_string();

    let result = esi_client.characters().character_affiliation(vec![0]).await;

    mock.assert();

    match result {
        Ok(_) => panic!("Expected Err"),
        Err(eve_esi::error::EsiError::ReqwestError(reqwest_error)) => {
            assert!(reqwest_error.status().is_some());
            assert_eq!(
                reqwest_error.status().unwrap(),
                reqwest::StatusCode::BAD_REQUEST
            );
        }
        Err(_) => panic!("Expected EsiError::ReqwestError with status code 400"),
    }
}
