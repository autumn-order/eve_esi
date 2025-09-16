use crate::util::integration_test_setup;

public_endpoint_test! {
    get_character_corporation_history,
    |esi_client: eve_esi::Client | async move {
        let character_ids = vec![2114794365, 2117053828];
        esi_client
            .character()
            .character_affiliation(character_ids)
            .await
    },
    request_type = "POST",
    mock_response = serde_json::json!([
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
    ]),
    url = "/characters/affiliation/"
}
