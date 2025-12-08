use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};
use crate::util::integration_test_setup;
use eve_esi::{scope::CharactersScopes, ScopeBuilder};

public_esi_request_test! {
    get_character_public_information,
    character,
    get_character_public_information[2114794365],
    request_type = "GET",
    url = "/characters/2114794365",
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
    })
}

public_esi_request_test! {
    character_affiliation,
    character,
    character_affiliation[vec![2114794365, 2117053828]],
    request_type = "POST",
    url = "/characters/affiliation",
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
    ])
}

authenticated_esi_request_test! {
    get_agents_research,
    character,
    get_agents_research[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/agents_research",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_agents_research())
        .build();
    mock_response = serde_json::json!([{
        "agent_id": 100,
        "points_per_day": 1.07832178,
        "remainder_points": 1.07832178,
        "skill_type_id": 100,
        "started_at": "2018-12-20T16:11:54Z",
    }]),
}

authenticated_esi_request_test! {
    get_blueprints,
    character,
    get_blueprints[2114794365, 0],
    request_type = "GET",
    url = "/characters/2114794365/blueprints?page=0",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_blueprints())
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

public_esi_request_test! {
    get_corporation_history,
    character,
    get_corporation_history[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/corporationhistory",
    mock_response = serde_json::json!([
        {
            "corporation_id": 1,
            "record_id": 1,
            "start_date": "2018-12-20T16:11:54Z"
        }
    ])
}

authenticated_esi_request_test! {
    calculate_a_cspa_charge_cost,
    character,
    calculate_a_cspa_charge_cost[2114794365, vec![2117053828]],
    request_type = "POST",
    url = "/characters/2114794365/cspa",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_contacts())
        .build();
    mock_response = serde_json::json!(5000000),
}

authenticated_esi_request_test! {
    get_jump_fatigue,
    character,
    get_jump_fatigue[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/fatigue",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_fatigue())
        .build();
    mock_response = serde_json::json!({
        "jump_fatigue_expire_date": "2018-12-20T16:11:54Z",
        "last_jump_date": "2018-12-20T16:11:54Z",
        "last_update_date": "2018-12-20T16:11:54Z",
    }),
}

authenticated_esi_request_test! {
    get_medals,
    character,
    get_medals[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/medals",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_medals())
        .build();
    mock_response = serde_json::json!([
        {
            "corporation_id": 98785281,
            "date": "2018-12-20T16:11:54Z",
            "description": "medal description",
            "graphics": [
                {
                    "color": 1,
                    "graphic": "graphic name",
                    "layer": 1,
                    "part": 1
                }
            ],
            "issuer_id": 2114794365,
            "medal_id": 1,
            "reason": "Reason medal was issued",
            "status": "public",
            "title": "Medal name"
        }
    ]),
}

authenticated_esi_request_test! {
    get_character_notifications,
    character,
    get_character_notifications[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/notifications",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_notifications())
        .build();
    mock_response = serde_json::json!([
      {
        "is_read": true,
        "notification_id": 0,
        "sender_id": 0,
        "sender_type": "character",
        "text": "string",
        "timestamp": "2019-08-24T14:15:22Z",
        "type": "AcceptedAlly"
      }
    ]),
}

authenticated_esi_request_test! {
    get_new_contact_notifications,
    character,
    get_new_contact_notifications[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/notifications/contacts",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_notifications())
        .build();
    mock_response = serde_json::json!([
      {
        "message": "string",
        "notification_id": 0,
        "send_date": "2019-08-24T14:15:22Z",
        "sender_character_id": 0,
        "standing_level": 0
      }
    ]),
}

public_esi_request_test! {
    get_character_portraits,
    character,
    get_character_portraits[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/portrait",
    mock_response = serde_json::json!({
        "px64x64": "ABCD",
        "px128x128": "ABCD",
        "px256x256": "ABCD",
        "px512x512": "ABCD",
    })
}

authenticated_esi_request_test! {
    read_corporation_roles,
    character,
    get_character_corporation_roles[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/roles",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_corporation_roles())
        .build();
    mock_response = serde_json::json!({
        "roles": ["Brand_Manager"],
        "roles_at_base": ["Brand_Manager"],
        "roles_at_hq": ["Brand_Manager"],
        "roles_at_other": ["Brand_Manager"],
    }),
}

authenticated_esi_request_test! {
    get_standings,
    character,
    get_standings[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/standings",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_standings())
        .build();
    mock_response = serde_json::json!([{
        "from_id": 1,
        "from_type": "npc_corp",
        "standing": -0.12312385
    }]),
}

authenticated_esi_request_test! {
    get_character_corporation_titles,
    character,
    get_character_corporation_titles[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/titles",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_titles())
        .build();
    mock_response = serde_json::json!([{
        "name": "Title",
        "title_id": 1
    }]),
}
