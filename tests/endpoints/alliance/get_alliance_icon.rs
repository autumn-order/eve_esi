use eve_esi::model::alliance::AllianceIcons;

use crate::util::setup;

/// Successful retrieval of alliance icons
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock alliance icons
/// - Configure mock server with endpoint returning mock alliance icons
///
/// # Assertions
/// - Assert 1 request was made to the mock server
/// - Assert result is Ok
#[tokio::test]
async fn test_get_alliance_icon_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock alliance icons
    let mock_icons = AllianceIcons {
        px128x128: "ABCD".to_string(),
        px64x64: "ABCD".to_string(),
    };

    // Configure mock server with endpoint returning mock alliance icons
    let mock = mock_server
        .mock("GET", "/alliances/99013534/icons")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_icons).unwrap())
        .create();

    // Retrieve the list of corporations part of alliance 99013534
    let result = esi_client.alliance().get_alliance_icon(99013534).await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());
}

/// Receiving an error 404 when attempting to retrieve alliance icons
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
async fn test_get_alliance_icon_not_found() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Configure a mock response returning a 404 not found
    let mock = mock_server
        .mock("GET", "/alliances/99013534/icons")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Alliance not found"}"#)
        .create();

    // Retrieve the list of corporations part of alliance 99013534
    let result = esi_client.alliance().get_alliance_icon(99013534).await;

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is error
    assert!(result.is_err());

    // Assert error is due to not found error
    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
