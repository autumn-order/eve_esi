use crate::util::integration_test_setup;

public_endpoint_test! {
    list_all_alliances,
    |esi_client: eve_esi::Client | async move {
        esi_client
            .alliance()
            .list_all_alliances()
            .await
    },
    request_type = "GET",
    url = "/alliances",
    mock_response = serde_json::json!([1, 2, 3, 4, 5, 6, 7, 8, 9])
}

public_endpoint_test! {
    get_alliance_information,
    |esi_client: eve_esi::Client | async move {
        let alliance_id = 99013534;
        esi_client
            .alliance()
            .get_alliance_information(alliance_id)
            .await
    },
    request_type = "GET",
    url = "/alliances/99013534",
    mock_response = serde_json::json!({
        "creator_corporation_id": 98784257,
        "creator_id": 2114794365,
        "faction_id": null,
        "date_founded": "2024-09-25T06:25:58Z",
        "executor_corporation_id": 98787881,
        "name": "Autumn.",
        "ticker": "AUTMN",
    })
}

public_endpoint_test! {
    list_alliance_corporations,
    |esi_client: eve_esi::Client | async move {
        let alliance_id = 99013534;
        esi_client
            .alliance()
            .list_alliance_corporations(alliance_id)
            .await
    },
    request_type = "GET",
    url = "/alliances/99013534/corporations",
    mock_response = serde_json::json!([1, 2, 3, 4, 5, 6, 7, 8, 9])
}

public_endpoint_test! {
    get_alliance_icon,
    |esi_client: eve_esi::Client | async move {
        let alliance_id = 99013534;
        esi_client
            .alliance()
            .get_alliance_icon(alliance_id)
            .await
    },
    request_type = "GET",
    url = "/alliances/99013534/icons",
    mock_response = serde_json::json!({
        "px128x128": "ABCD",
        "px64x64":"ABCD"
    })
}
