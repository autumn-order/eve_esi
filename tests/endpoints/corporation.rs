use eve_esi::oauth2::scope::CorporationScopes;
use eve_esi::ScopeBuilder;

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};
use crate::util::integration_test_setup;

public_endpoint_test! {
    get_npc_corporations,
    |esi_client: eve_esi::Client | async move {
        esi_client
            .corporation()
            .get_npc_corporations()
            .await
    },
    request_type = "GET",
    url = "/corporations/npccorps",
    mock_response = serde_json::json!([98785281])
}

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
    url = "/corporations/98785281",
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
    })
}

public_endpoint_test! {
    get_alliance_history,
    |esi_client: eve_esi::Client | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_alliance_history(corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/alliancehistory",
    mock_response = serde_json::json!([
        {
            "alliance_id": 1,
            "record_id": 1,
            "start_date": "2018-12-20T16:11:54Z"
        }
    ])
}

authenticated_endpoint_test! {
    get_corporation_blueprints,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_blueprints(&access_token, corporation_id, page)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/blueprints?page=1",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_blueprints())
        .build();
    mock_response = serde_json::json!([{
        "item_id": 0,
        "location_flag": "Hangar",
        "location_id": 0,
        "material_efficiency": 0,
        "quantity": -1,
        "runs": -1,
        "time_efficiency": 0,
        "type_id": 0
    }]),
}
