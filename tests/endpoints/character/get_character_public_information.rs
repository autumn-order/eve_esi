use eve_esi::model::character::Character;

use crate::util::setup;

/// Tests the successful retrieval of character information from a mock EVE ESI server.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock character data
/// - Configure mock server with an ESI endpoint returning the mock character data
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is Ok
/// - Assert received expected character data
#[tokio::test]
async fn get_character_public_information() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock character data
    let mock_character = Character {
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
        title: Some("Title".to_string()),
    };

    // Configure mock server with an ESI endpoint returning the mock character data
    let mock = mock_server
        .mock("GET", "/characters/2114794365/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_character).unwrap())
        .create();

    // Retrieve the character data
    let result = esi_client
        .character()
        .get_character_public_information(2114794365)
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Assert received expected character data
    let character = result.unwrap();
    assert_eq!(character, mock_character);
}

/// Tests receiving a 404 error when attempting to retrieve character information from a mock EVE ESI server.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Configure mock server with an ESI endpoint returning 404 not found
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is error
/// - Assert reqwest error is due to status NOT_FOUND
#[tokio::test]
async fn get_character_public_information_not_found() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Configure mock server with an ESI endpoint returning 404 not found
    let mock = mock_server
        .mock("GET", "/characters/2114794365/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Character not found"}"#)
        .create();

    // Retrieve the character data
    let result = esi_client
        .character()
        .get_character_public_information(2114794365)
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is error
    assert!(result.is_err());
    match result {
        Err(eve_esi::Error::ReqwestError(err)) => {
            // Assert reqwest error is due to status NOT_FOUND
            assert!(err.status().is_some());
            assert_eq!(err.status().unwrap(), reqwest::StatusCode::NOT_FOUND);
        }
        err => {
            panic!(
                "Expected ReqwestError, got different error type: {:#?}",
                err
            )
        }
    }
}
