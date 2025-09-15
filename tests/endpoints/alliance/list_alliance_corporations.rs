use crate::util::integration_test_setup;

/// Successful retrieval of IDs of corporations part of an alliance
#[tokio::test]
async fn test_list_alliance_corporations_success() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_alliance_corporation_ids = serde_json::json!([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mock_alliance_corporations_endpoint = mock_server
        .mock("GET", "/alliances/99013534/corporations")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_alliance_corporation_ids.to_string())
        .create();

    let alliance_id = 99013534;
    let result = esi_client
        .alliance()
        .list_alliance_corporations(alliance_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_alliance_corporations_endpoint.assert();

    assert!(result.is_ok());
}

/// Receiving an error 404 when calling list_alliance_corporations()
#[tokio::test]
async fn test_list_alliance_corporations_not_found() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_alliance_corporations_endpoint = mock_server
        .mock("GET", "/alliances/99013534/corporations")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Alliance not found"}"#)
        .create();

    let alliance_id = 99013534;
    let result = esi_client
        .alliance()
        .list_alliance_corporations(alliance_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_alliance_corporations_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
