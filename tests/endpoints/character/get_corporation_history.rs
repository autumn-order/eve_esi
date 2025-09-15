use crate::util::integration_test_setup;

/// Successful retrieval of a character's corporation history
#[tokio::test]
async fn test_get_character_corporation_history() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_corporation_history = serde_json::json!([
        {
            "corporation_id": 98785281,
            "is_deleted": false,
            "record_id": 0,
            "start_date": "2024-10-07T21:43:09Z"
        }
    ]);

    let mock_character_corporation_history_endpoint = mock_server
        .mock("GET", "/characters/2114794365/corporationhistory")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_corporation_history.to_string())
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .get_corporation_history(character_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_character_corporation_history_endpoint.assert();

    assert!(result.is_ok());
}

/// Failed retrieval of character information due to 404 not found error
#[tokio::test]
async fn test_get_character_corporation_history_not_found() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_character_endpoint = mock_server
        .mock("GET", "/characters/2114794365/corporationhistory")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Character not found"}"#)
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .get_corporation_history(character_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_character_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
