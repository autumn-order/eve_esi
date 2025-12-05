use eve_esi::scope::{CorporationsScopes, WalletScopes};
use eve_esi::ScopeBuilder;

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};
use crate::util::integration_test_setup;

public_esi_request_test! {
    get_npc_corporations,
    |esi_client: eve_esi::Client | {
        esi_client
            .corporation()
            .get_npc_corporations()
    },
    request_type = "GET",
    url = "/corporations/npccorps",
    mock_response = serde_json::json!([98785281])
}

public_esi_request_test! {
    get_corporation,
    |esi_client: eve_esi::Client | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_information(corporation_id)
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

public_esi_request_test! {
    get_alliance_history,
    |esi_client: eve_esi::Client | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_alliance_history(corporation_id)
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

authenticated_esi_request_test! {
    get_corporation_blueprints,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_blueprints(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/blueprints?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_blueprints())
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

authenticated_esi_request_test! {
    get_all_corporation_alsc_logs,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_all_corporation_alsc_logs(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/containers/logs?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_container_logs())
        .build();
    mock_response = serde_json::json!([{
        "action": "add",
        "character_id": 2114794365,
        "container_id": 0,
        "container_type_id": 0,
        "location_flag": "Hangar",
        "location_id": 0,
        "logged_at": "2018-12-20T16:11:54Z",
        "new_config_bitmask": 0,
        "old_config_bitmask": 0,
        "password_type": "string",
        "quantity": 0,
        "type_id": 0
    }]),
}

authenticated_esi_request_test! {
    get_corporation_divisions,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_divisions(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/divisions",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_divisions())
        .build();
    mock_response = serde_json::json!({
        "hangar": [
            {
                "division": 1,
                "name": "string"
            }
        ],
        "wallet": [
            {
                "division": 1,
                "name": "string"
            }
        ]
    }),
}

authenticated_esi_request_test! {
    get_corporation_facilities,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_facilities(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/facilities",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_facilities())
        .build();
    mock_response = serde_json::json!([{
        "facility_id": 0,
        "system_id": 0,
        "type_id": 0
    }]),
}

public_esi_request_test! {
    get_corporation_icon,
    |esi_client: eve_esi::Client | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_icon(corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/icons",
    mock_response = serde_json::json!({
        "px128x128": "string",
        "px256x256": "string",
        "px64x64": "string"
    })
}

authenticated_esi_request_test! {
    get_corporation_medals,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_medals(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/medals?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_medals())
        .build();
    mock_response = serde_json::json!([{
        "created_at": "2018-12-20T16:11:54Z",
        "creator_id": 2114794365,
        "description": "string",
        "medal_id": 0,
        "title": "string"
    }]),
}

authenticated_esi_request_test! {
    get_corporation_issued_medals,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_issued_medals(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/medals/issued?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_medals())
        .build();
    mock_response = serde_json::json!([{
        "character_id": 2114794365,
        "issued_at": "2018-12-20T16:11:54Z",
        "issuer_id": 2114794365,
        "medal_id": 0,
        "reason": "string",
        "status": "private"
    }]),
}

authenticated_esi_request_test! {
    get_corporation_members,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_members(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/members",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!([2114794365]),
}

authenticated_esi_request_test! {
    get_corporation_member_limit,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_member_limit(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/members/limit",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!(40),
}

authenticated_esi_request_test! {
    get_corporation_members_titles,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_members_titles(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/members/titles",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_titles())
        .build();
    mock_response = serde_json::json!([{
        "character_id": 2114794365,
        "titles": [0]
    }]),
}

authenticated_esi_request_test! {
    track_corporation_members,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .track_corporation_members(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/membertracking",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().track_members())
        .build();
    mock_response = serde_json::json!([{
        "base_id": 0,
        "character_id": 2114794365,
        "location_id": 0,
        "logoff_date": "2018-12-20T16:11:54Z",
        "logon_date": "2018-12-20T16:11:54Z",
        "ship_type_id": 0,
        "start_date": "2018-12-20T16:11:54Z"
    }]),
}

authenticated_esi_request_test! {
    get_corporation_member_roles,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_member_roles(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/roles",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!([{
        "character_id": 2114794365,
        "grantable_roles": ["Accountant"],
        "grantable_roles_at_base": ["Accountant"],
        "grantable_roles_at_hq": ["Accountant"],
        "grantable_roles_at_other": ["Accountant"],
        "roles": ["Accountant"],
        "roles_at_base": ["Accountant"],
        "roles_at_hq": ["Accountant"],
        "roles_at_other": ["Accountant"]
    }]),
}

authenticated_esi_request_test! {
    get_corporation_member_roles_history,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_member_roles_history(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/roles/history?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!([{
        "changed_at": "2018-12-20T16:11:54Z",
        "character_id": 2114794365,
        "issuer_id": 2114794365,
        "new_roles": ["Accountant"],
        "old_roles": ["Accountant"],
        "role_type": "roles"
    }]),
}

authenticated_esi_request_test! {
    get_corporation_shareholders,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_shareholders(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/shareholders?page=1",
    required_scopes = ScopeBuilder::new()
        .wallet(WalletScopes::new().read_corporation_wallets())
        .build();
    mock_response = serde_json::json!([{
        "share_count": 0,
        "shareholder_id": 2114794365,
        "shareholder_type": "character"
    }]),
}

authenticated_esi_request_test! {
    get_corporation_standings,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_standings(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/standings?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_standings())
        .build();
    mock_response = serde_json::json!([{
        "from_id": 0,
        "from_type": "agent",
        "standing": 0.0
    }]),
}

authenticated_esi_request_test! {
    get_corporation_starbases,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_starbases(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/starbases?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_starbases())
        .build();
    mock_response = serde_json::json!([{
        "moon_id": 0,
        "onlined_since": "2018-12-20T16:11:54Z",
        "reinforced_until": "2018-12-20T16:11:54Z",
        "starbase_id": 0,
        "state": "offline",
        "system_id": 0,
        "type_id": 0,
        "unanchors_at": "2018-12-20T16:11:54Z"
    }]),
}

authenticated_esi_request_test! {
    get_starbase_detail,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let starbase_id = 12345;
        let system_id = 30000142;
        esi_client
            .corporation()
            .get_starbase_detail(&access_token, corporation_id, starbase_id, system_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/starbases/12345?system_id=30000142",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_starbases())
        .build();
    mock_response = serde_json::json!({
        "allow_alliance_members": true,
        "allow_corporation_members": true,
        "anchor": "config_starbase_equipment_role",
        "attack_if_at_war": true,
        "attack_if_other_security_status_dropping": true,
        "attack_security_status_threshold": 0.0,
        "attack_standing_threshold": 0.0,
        "fuel_bay_take": "config_starbase_equipment_role",
        "fuel_bay_view": "config_starbase_equipment_role",
        "fuels": [
            {
                "quantity": 0,
                "type_id": 0
            }
        ],
        "offline": "config_starbase_equipment_role",
        "online": "config_starbase_equipment_role",
        "unanchor": "config_starbase_equipment_role",
        "use_alliance_standings": true
    }),
}

authenticated_esi_request_test! {
    get_corporation_structures,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .corporation()
            .get_corporation_structures(&access_token, corporation_id, page)
    },
    request_type = "GET",
    url = "/corporations/98785281/structures?page=1",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_structures())
        .build();
    mock_response = serde_json::json!([{
        "corporation_id": 98785281,
        "fuel_expires": "2018-12-20T16:11:54Z",
        "next_reinforce_apply": "2018-12-20T16:11:54Z",
        "next_reinforce_hour": 0,
        "profile_id": 0,
        "reinforce_hour": 0,
        "services": [
            {
                "name": "string",
                "state": "online"
            }
        ],
        "state": "anchor_vulnerable",
        "state_timer_end": "2018-12-20T16:11:54Z",
        "state_timer_start": "2018-12-20T16:11:54Z",
        "structure_id": 0,
        "system_id": 0,
        "type_id": 0,
        "unanchors_at": "2018-12-20T16:11:54Z"
    }]),
}

authenticated_esi_request_test! {
    get_corporation_titles,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .corporation()
            .get_corporation_titles(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/titles",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_titles())
        .build();
    mock_response = serde_json::json!([{
        "grantable_roles": ["Accountant"],
        "grantable_roles_at_base": ["Accountant"],
        "grantable_roles_at_hq": ["Accountant"],
        "grantable_roles_at_other": ["Accountant"],
        "name": "string",
        "roles": ["Accountant"],
        "roles_at_base": ["Accountant"],
        "roles_at_hq": ["Accountant"],
        "roles_at_other": ["Accountant"],
        "title_id": 0
    }]),
}
