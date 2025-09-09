use crate::util::setup;

/// Successful retrieval of list of alliance IDs
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock Vec of alliance IDs
/// - Configure mock server with an ESI endpoint a mock list of alliance IDs
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is Ok
#[tokio::test]
async fn test_list_all_alliances_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock Vec of alliance IDs
    let mock_alliance = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Configure mock server with an ESI endpoint a mock list of alliance IDs
    let mock = mock_server
        .mock("GET", "/alliances")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_alliance).unwrap())
        .create();

    // Retrieve the list of alliances
    let result = esi_client.alliance().list_all_alliances().await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());
}

/// Receiving an error 500 when calling list_all_alliances()
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Configure mock server with an ESI endpoint returning a 600 internal server error
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is error
/// - Assert error is due to internal server error
#[tokio::test]
async fn test_list_all_alliances_internal_error() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Configure a mock response returning a 500 internal server error
    let mock = mock_server
        .mock("GET", "/alliances")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    // Retrieve the list of alliances
    let result = esi_client.alliance().list_all_alliances().await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is error
    assert!(result.is_err());

    // Assert error is due to internal server error
    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
    );
}
