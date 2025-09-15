use crate::util::integration_test_setup;

/// Successful retrieval of alliance icons
#[tokio::test]
async fn test_get_alliance_icon_success() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_alliance_icons = serde_json::json!({
        "px128x128": "ABCD",
        "px64x64":"ABCD"
    });

    let mock_icons_endpoint = mock_server
        .mock("GET", "/alliances/99013534/icons")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_alliance_icons.to_string())
        .create();

    let result = esi_client.alliance().get_alliance_icon(99013534).await;

    // Assert 1 request was made to the mock endpoint
    mock_icons_endpoint.assert();

    assert!(result.is_ok());
}

/// Receiving an error 404 when attempting to retrieve alliance icons
#[tokio::test]
async fn test_get_alliance_icon_not_found() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_icons_endpoint = mock_server
        .mock("GET", "/alliances/99013534/icons")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Alliance not found"}"#)
        .create();

    let result = esi_client.alliance().get_alliance_icon(99013534).await;

    // Assert 1 request was made to the mock endpoint
    mock_icons_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
