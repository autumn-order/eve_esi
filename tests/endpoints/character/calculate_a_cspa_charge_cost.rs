use crate::util::integration_test_setup;

public_endpoint_test! {
    calculate_a_cspa_charge_cost,
    |esi_client: eve_esi::Client | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .calculate_a_cspa_charge_cost(character_id)
            .await
    },
    request_type = "GET",
    mock_response = serde_json::json!([5000000]),
    url = "/characters/2114794365/cspa"
}
