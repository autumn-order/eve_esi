use crate::util::integration_test_setup;

/// Successful retrieval of character affiliations
#[tokio::test]
async fn character_affiliation() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_character_affiliations = serde_json::json!([
        {
            "character_id": 2114794365,
            "corporation_id": 98785281,
            "alliance_id": 99013534,
            "faction_id": null,
        },
        {
            "character_id": 2117053828,
            "corporation_id": 98785281,
            "alliance_id": 99013534,
            "faction_id": null,
        },
    ]);

    let mock_character_affiliations_endpoint = mock_server
        .mock("POST", "/characters/affiliation/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_character_affiliations.to_string())
        .create();

    let character_ids = vec![2114794365, 2117053828];
    let result = esi_client
        .character()
        .character_affiliation(character_ids)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_character_affiliations_endpoint.assert();

    assert!(result.is_ok());
}

/// Failed retrieval of character affiliations due to a bad request error
#[tokio::test]
async fn character_affiliation_bad_request() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mock_character_affiliations_endpoint = mock_server
        .mock("POST", "/characters/affiliation/")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "error": "Invalid character ID"
            }"#,
        )
        .create();

    let character_ids = vec![0];
    let result = esi_client
        .character()
        .character_affiliation(character_ids)
        .await;

    // Assert 1 request was made to the mock endpoint
    mock_character_affiliations_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::BAD_REQUEST))
    );
}
