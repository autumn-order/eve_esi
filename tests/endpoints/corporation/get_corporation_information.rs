use eve_esi::error::EsiError;
use eve_esi::model::corporation::Corporation;

use super::super::util::setup;

/// Tests the successful retrieval of a corporation from a mock EVE ESI server.
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock corporation data
/// - Configure mock server with an ESI endpoint returning the mock corporation
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is Ok
/// - Assert received expected corporation data
#[tokio::test]
async fn get_corporation() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock corporation data
    let mock_corporation = Corporation {
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

    // Configure mock server with an ESI endpoint returning the mock corporation data
    let mock = mock_server
        .mock("GET", "/corporations/98785281/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_corporation).unwrap())
        .create();

    // Retrieve the corporation data
    let result = esi_client
        .corporation()
        .get_corporation_information(98785281)
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Assert received expected corporation data
    let corporation = result.unwrap();
    assert_eq!(corporation, mock_corporation);
}

/// Tests the failed retrieval of corporation due to a 404 not found error.
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
async fn get_corporation_not_found() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Configure mock server with an ESI endpoint returning 404 not found
    let mock = mock_server
        .mock("GET", "/corporations/99999999/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Corporation not found"}"#)
        .create();

    // Retrieve the corporation data
    let result = esi_client
        .corporation()
        .get_corporation_information(99999999)
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
