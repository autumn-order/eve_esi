use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};
use crate::util::integration_test_setup;
use eve_esi::scope::{CorporationsScopes, WalletScopes};
use eve_esi::ScopeBuilder;

public_esi_request_test! {
    get_npc_corporations,
    corporation,
    get_npc_corporations[],
    request_type = "GET",
    url = "/corporations/npccorps",
    mock_response = serde_json::json!([98785281])
}

public_esi_request_test! {
    get_corporation,
    corporation,
    get_corporation_information[98785281],
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
    corporation,
    get_alliance_history[98785281],
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
    corporation,
    get_corporation_blueprints[98785281, 1],
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
    corporation,
    get_all_corporation_alsc_logs[98785281, 1],
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
    corporation,
    get_corporation_divisions[98785281],
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
    corporation,
    get_corporation_facilities[98785281],
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
    corporation,
    get_corporation_icon[98785281],
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
    corporation,
    get_corporation_medals[98785281, 1],
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
    corporation,
    get_corporation_issued_medals[98785281, 1],
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
    corporation,
    get_corporation_members[98785281],
    request_type = "GET",
    url = "/corporations/98785281/members",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!([2114794365]),
}

authenticated_esi_request_test! {
    get_corporation_member_limit,
    corporation,
    get_corporation_member_limit[98785281],
    request_type = "GET",
    url = "/corporations/98785281/members/limit",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_corporation_membership())
        .build();
    mock_response = serde_json::json!(40),
}

authenticated_esi_request_test! {
    get_corporation_members_titles,
    corporation,
    get_corporation_members_titles[98785281],
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
    corporation,
    track_corporation_members[98785281],
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
    corporation,
    get_corporation_member_roles[98785281],
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
    corporation,
    get_corporation_member_roles_history[98785281, 1],
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
    corporation,
    get_corporation_shareholders[98785281, 1],
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
    corporation,
    get_corporation_standings[98785281, 1],
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
    corporation,
    get_corporation_starbases[98785281, 1],
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
    corporation,
    get_starbase_detail[98785281, 12345, 30000142],
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
    corporation,
    get_corporation_structures[98785281, 1],
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
    corporation,
    get_corporation_titles[98785281],
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
