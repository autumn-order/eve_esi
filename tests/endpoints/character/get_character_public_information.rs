use crate::util::setup;

/// Tests the successful retrieval of character information from a mock EVE ESI server.
#[tokio::test]
async fn get_character_public_information() {
    let (esi_client, mut mock_server) = setup().await;

    let mock_character = serde_json::json!({
        "alliance_id": 99013534,
        "birthday": "2018-12-20T16:11:54Z",
        "bloodline_id": 7,
        "corporation_id": 98785281,
        "description": "description",
        "faction_id": null,
        "gender": "male",
        "name": "Hyziri",
        "race_id": 8,
        "security_status": -0.100373643,
        "title": "Title",
    });

    let mock_character_endpoint = mock_server
        .mock("GET", "/characters/2114794365/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_character.to_string())
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .get_character_public_information(character_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_character_endpoint.assert();

    assert!(result.is_ok());
}

/// Failed retrieval of character information due to 404 not found error
#[tokio::test]
async fn get_character_public_information_not_found() {
    let (esi_client, mut mock_server) = setup().await;

    let mock_character_endpoint = mock_server
        .mock("GET", "/characters/2114794365/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Character not found"}"#)
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .get_character_public_information(character_id)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_character_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
}
