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

authenticated_endpoint_test! {
    get_all_corporation_alsc_logs,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_all_corporation_alsc_logs(&access_token, corporation_id, page)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/containers/logs?page=1",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_container_logs())
        .build();
    mock_response = serde_json::json!([{
        "action": "enter_password",
        "character_id": 2114794365,
        "container_id": 1,
        "container_type_id": 1,
        "location_flag": "Hangar",
        "location_id": 1,
        "logged_at": "2018-12-20T16:11:54Z",
        "new_config_bitmask": 1,
        "old_config_bitmask": 1,
        "quantity": 1,
        "type_id": 1
    }]),
}

authenticated_endpoint_test! {
    get_corporation_divisions,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_divisions(&access_token, corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/divisions",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_divisions())
        .build();
    mock_response = serde_json::json!({
        "hangar": [{
            "division": 1,
            "name": "Hangar 1"
        }],
        "wallet": [{
            "division": 1,
            "name": "Master wallet"
        }]
    }),
}

authenticated_endpoint_test! {
    get_corporation_facilities,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_facilities(&access_token, corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/facilities",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_facilities())
        .build();
    mock_response = serde_json::json!([
        {
            "facility_id": 1,
            "system_id": 1,
            "type_id": 1
        }
    ]),
}

public_endpoint_test! {
    get_corporation_icon,
    |esi_client: eve_esi::Client | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_icon(corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/icons",
    mock_response = serde_json::json!({
        "px128x128": "ABCD",
        "px256x256": "ABCD",
        "px64x64": "ABCD"
    })
}

authenticated_endpoint_test! {
    get_corporation_medals,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_medals(&access_token, corporation_id, page)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/medals?page=1",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_medals())
        .build();
    mock_response = serde_json::json!([
        {
            "created_at": "2018-12-20T16:11:54Z",
            "creator_id": 2114794365,
            "description": "Medal description",
            "medal_id": 1,
            "title": "Medal name"
        }
    ]),
}

authenticated_endpoint_test! {
    get_corporation_issued_medals,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_issued_medals(&access_token, corporation_id, page)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/medals/issued?page=1",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_medals())
        .build();
    mock_response = serde_json::json!([
        {
            "character_id": 2114794365,
            "issued_at": "2018-12-20T16:11:54Z",
            "issuer_id": 2114794365,
            "medal_id": 1,
            "reason": "Reason medal was issued",
            "status": "public"
        }
    ]),
}

public_endpoint_test! {
    get_corporation_members,
    |esi_client: eve_esi::Client | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_members(corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/members",
    mock_response = serde_json::json!([2114794365, 2117053828])
}

authenticated_endpoint_test! {
    get_corporation_member_limit,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_member_limit(&access_token, corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/members/limit",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().track_members())
        .build();
    mock_response = serde_json::json!(20),
}

authenticated_endpoint_test! {
    get_corporation_member_titles,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_members_titles(&access_token, corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/members/titles",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_titles())
        .build();
    mock_response = serde_json::json!([
        {
            "character_id": 2114794365,
            "titles": [1, 2, 3, 4, 5]
        }
    ]),
}

authenticated_endpoint_test! {
    track_corporation_members,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .track_corporation_members(&access_token, corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/membertracking",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().track_members())
        .build();
    mock_response = serde_json::json!([
        {
            "base_id": 1,
            "character_id": 2114794365,
            "location_id": 1,
            "logoff_date": "2018-12-20T16:11:54Z",
            "logon_date": "2018-12-20T16:11:54Z",
            "ship_type_id": 1,
            "start_date": "2018-12-20T16:11:54Z"
        }
    ]),
}

authenticated_endpoint_test! {
    get_corporation_member_roles,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_member_roles(&access_token, corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/roles",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!([
      {
        "character_id": 0,
        "grantable_roles": [
          "Account_Take_1"
        ],
        "grantable_roles_at_base": [
          "Account_Take_1"
        ],
        "grantable_roles_at_hq": [
          "Account_Take_1"
        ],
        "grantable_roles_at_other": [
          "Account_Take_1"
        ],
        "roles": [
          "Account_Take_1"
        ],
        "roles_at_base": [
          "Account_Take_1"
        ],
        "roles_at_hq": [
          "Account_Take_1"
        ],
        "roles_at_other": [
          "Account_Take_1"
        ]
      }
    ]),
}

authenticated_endpoint_test! {
    get_corporation_member_roles_history,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_member_roles_history(&access_token, corporation_id, page)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/roles/history?page=1",
    required_scopes = ScopeBuilder::new()
        .corporation(CorporationScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!([
      {
        "changed_at": "2019-08-24T14:15:22Z",
        "character_id": 0,
        "issuer_id": 0,
        "new_roles": [
          "Account_Take_1"
        ],
        "old_roles": [
          "Account_Take_1"
        ],
        "role_type": "grantable_roles"
      }
    ]),
}
