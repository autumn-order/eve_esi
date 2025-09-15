use crate::util::integration_test_setup;

/// Successful retrieval of list of alliance IDs
#[tokio::test]
async fn test_list_all_alliances_success() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_alliance_ids = serde_json::json!([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mock_alliance_endpoint = mock_server
        .mock("GET", "/alliances")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_alliance_ids.to_string())
        .create();

    let result = esi_client.alliance().list_all_alliances().await;

    // Assert 1 request was made to the mock endpoint
    mock_alliance_endpoint.assert();

    assert!(result.is_ok());
}

/// Receiving an error 500 when calling list_all_alliances()
#[tokio::test]
async fn test_list_all_alliances_internal_error() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock = mock_server
        .mock("GET", "/alliances")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    let result = esi_client.alliance().list_all_alliances().await;

    // Assert 1 request was made to the mock endpoint
    mock.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
    );
}
