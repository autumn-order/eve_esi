use crate::util::integration_test_setup;

public_endpoint_test! {
    get_character_public_information,
    |esi_client: eve_esi::Client | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_character_public_information(character_id)
            .await
    },
    request_type = "GET",
    mock_response = serde_json::json!({
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
    }),
    url = "/characters/2114794365/"
}
