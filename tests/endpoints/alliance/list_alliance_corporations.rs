use crate::util::setup;

/// Successful retrieval of IDs of corporations part of an alliance
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock Vec of corporation IDs
/// - Configure mock server with an ESI endpoint returning a mock list of corporation IDs
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is Ok
#[tokio::test]
async fn test_list_alliance_corporations_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock Vec of corporation IDs
    let mock_corporation_ids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Configure mock server with an ESI endpoint returning a mock list of corporation IDs
    let mock = mock_server
        .mock("GET", "/alliances/99013534/corporations")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_corporation_ids).unwrap())
        .create();

    // Retrieve the list of corporations part of alliance 99013534
    let result = esi_client
        .alliance()
        .list_alliance_corporations(99013534)
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());
}

/// Receiving an error 404 when calling list_alliance_corporations()
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Configure a mock response returning a 404 not found
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is error
/// - Assert error is due to not found error
#[tokio::test]
async fn test_list_alliance_corporations_not_found() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Configure a mock response returning a 404 not found
    let mock = mock_server
        .mock("GET", "/alliances/99013534/corporations")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Alliance not found"}"#)
        .create();

    // Retrieve the list of corporations part of alliance 99013534
    let result = esi_client
        .alliance()
        .list_alliance_corporations(99013534)
        .await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is error
    assert!(result.is_err());

    // Assert error is due to not found error
    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
