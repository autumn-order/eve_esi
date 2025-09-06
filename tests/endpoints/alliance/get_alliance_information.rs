use eve_esi::error::EsiError;
use eve_esi::model::alliance::Alliance;

use super::super::util::setup;

/// Tests the successful retrieval of alliance information from a mock EVE ESI server.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock alliance data
/// - Configure mock server with an ESI endpoint returning the mock alliance
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is Ok
/// - Assert received expected alliance data
#[tokio::test]
async fn get_alliance_information() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock alliance data
    let mock_alliance = Alliance {
        creator_corporation_id: 98784257,
        creator_id: 2114794365,
        faction_id: None,
        date_founded: "2024-09-25T06:25:58Z".parse().unwrap(),
        executor_corporation_id: Some(98787881),
        name: "Autumn.".to_string(),
        ticker: "AUTMN".to_string(),
    };

    // Configure mock server with an ESI endpoint returning 404 not found
    let mock = mock_server
        .mock("GET", "/alliances/99013534/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_alliance).unwrap())
        .create();

    // Retrieve the alliance
    let result = esi_client
        .alliance()
        .get_alliance_information(99013534)
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Assert received expected alliance data
    let alliance = result.unwrap();
    assert_eq!(alliance, mock_alliance);
}

/// Tests receiving a 404 error when attempting to retrieve alliance information from a mock EVE ESI server.
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
async fn get_alliance_information_not_found() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Configure a mock response returning a 404 not found
    let mock = mock_server
        .mock("GET", "/alliances/99999999/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Alliance not found"}"#)
        .create();

    // Retrieve the alliance
    let result = esi_client
        .alliance()
        .get_alliance_information(99999999)
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is error
    assert!(result.is_err());
    match result {
        Err(EsiError::ReqwestError(err)) => {
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
