use crate::util::integration_test_setup;

/// Successful retrieval of corporation information
#[tokio::test]
async fn get_corporation() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_corporation = serde_json::json!({
        "alliance_id": 99013534,
        "ceo_id": 2114794365,
        "creator_id": 2114794365,
        "date_founded": "2024-10-07T21:43:09Z",
        "description": "",
        "home_station_id": 60003760,
        "member_count": 21,
        "name": "The Order of Autumn",
        "shares": 1000,
        "tax_rate": 0.0,
        "ticker": "F4LL.",
        "url": "https://autumn-order.com",
        "war_eligible": true,
        "faction_id": null,
    });

    let mock_corporation_endpoint = mock_server
        .mock("GET", "/corporations/98785281/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_corporation).unwrap())
        .create();

    let corporation_id = 98785281;
    let result = esi_client
        .corporation()
        .get_corporation_information(corporation_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_corporation_endpoint.assert();

    assert!(result.is_ok());
}

/// Failed retrieval of corporation due to a 404 not found error.
#[tokio::test]
async fn get_corporation_not_found() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_corporation_endpoint = mock_server
        .mock("GET", "/corporations/99999999/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Corporation not found"}"#)
        .create();

    let corporation_id = 99999999;
    let result = esi_client
        .corporation()
        .get_corporation_information(corporation_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_corporation_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
