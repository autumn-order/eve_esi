use crate::util::integration_test_setup;

public_endpoint_test! {
    get_corporation,
    |esi_client: eve_esi::Client | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_information(corporation_id)
            .await
    },
    request_type = "GET",
    mock_response = serde_json::json!({
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
    }),
    url = "/corporations/98785281"
}
