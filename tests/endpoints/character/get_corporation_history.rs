use crate::util::integration_test_setup;

public_endpoint_test! {
    get_character_corporation_history,
    |esi_client: eve_esi::Client | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_corporation_history(character_id)
            .await
    },
    request_type = "GET",
    mock_response = serde_json::json!([
        {
            "corporation_id": 98785281,
            "is_deleted": false,
            "record_id": 0,
            "start_date": "2024-10-07T21:43:09Z"
        }
    ]),
    url = "/characters/2114794365/corporationhistory"
}
