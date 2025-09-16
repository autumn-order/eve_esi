use crate::util::integration_test_setup;

/// Successful retrieval of a character's CSPA charge cost
#[tokio::test]
async fn test_calculate_a_cspa_charge_cost_success() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_cspa_charge_cost = serde_json::json!([5000000]);

    let mock_cspa_charge_cost_endpoint = mock_server
        .mock("GET", "/characters/2114794365/cspa")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_cspa_charge_cost.to_string())
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .calculate_a_cspa_charge_cost(character_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_cspa_charge_cost_endpoint.assert();

    assert!(result.is_ok());
}

/// Failed retrieval of a character's CSPA charge cost due to 404 not found error
#[tokio::test]
async fn test_calculate_a_cspa_charge_cost_not_found() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_cspa_charge_cost_endpoint = mock_server
        .mock("GET", "/characters/2114794365/cspa")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Character not found"}"#)
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .calculate_a_cspa_charge_cost(character_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_cspa_charge_cost_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
