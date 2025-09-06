use eve_esi::error::EsiError;
use eve_esi::model::character::CharacterAffiliation;

use super::super::util::setup;

/// Tests the successful retrieval of character affiliations from a mock EVE ESI server.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock character affiliation data
/// - Configure mock server with an ESI endpoint returning the mock character affiliations
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is Ok
/// - Assert received expected character affiliation data
#[tokio::test]
async fn character_affiliation() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock character affiliation data
    let mock_character_affiliations = vec![
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

    // Configure mock server with an ESI endpoint returning the mock character affiliations
    let mock = mock_server
        .mock("POST", "/characters/affiliation/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_character_affiliations).unwrap())
        .create();

    // Retrieve the character affiliations
    let result = esi_client
        .character()
        .character_affiliation(vec![2114794365, 2117053828])
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Assert received expected character affiliation data
    let character_affiliations = result.unwrap();
    assert_eq!(character_affiliations, mock_character_affiliations);
}

/// Tests the failed retrieval of character affiliations due to a bad request error
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Configure mock server with an ESI endpoint returning 400 bad request
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is error
/// - Assert reqwest error is due to status BAD_REQUEST
#[tokio::test]
async fn character_affiliation_bad_request() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Configure mock server with an ESI endpoint returning 400 bad request
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

    // Retrieve the character affiliations
    let result = esi_client.character().character_affiliation(vec![0]).await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is error
    assert!(result.is_err());
    match result {
        Err(EsiError::ReqwestError(err)) => {
            // Assert reqwest error is due to status BAD_REQUEST
            assert!(err.status().is_some());
            assert_eq!(err.status().unwrap(), reqwest::StatusCode::BAD_REQUEST);
        }
        err => {
            panic!(
                "Expected ReqwestError, got different error type: {:#?}",
                err
            )
        }
    }
}
