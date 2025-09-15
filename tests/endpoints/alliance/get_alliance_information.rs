use crate::util::setup;

/// Successful retrieval of alliance information
#[tokio::test]
async fn get_alliance_information() {
    let (esi_client, mut mock_server) = setup().await;

    let mock_alliance = serde_json::json!({
        "creator_corporation_id": 98784257,
        "creator_id": 2114794365,
        "faction_id": null,
        "date_founded": "2024-09-25T06:25:58Z",
        "executor_corporation_id": 98787881,
        "name": "Autumn.",
        "ticker": "AUTMN",
    });

    let mock_alliance_endpoint = mock_server
        .mock("GET", "/alliances/99013534/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_alliance.to_string())
        .create();

    let result = esi_client
        .alliance()
        .get_alliance_information(99013534)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_alliance_endpoint.assert();

    assert!(result.is_ok());
}

/// Receiving a 404 error when attempting to retrieve alliance information
#[tokio::test]
async fn get_alliance_information_not_found() {
    let (esi_client, mut mock_server) = setup().await;

    let mock_alliance_endpoint = mock_server
        .mock("GET", "/alliances/99999999/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Alliance not found"}"#)
        .create();

    let result = esi_client
        .alliance()
        .get_alliance_information(99999999)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_alliance_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
