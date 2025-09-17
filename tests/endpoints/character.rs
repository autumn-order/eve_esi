use eve_esi::{oauth2::scope::CharacterScopes, ScopeBuilder};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};
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

authenticated_endpoint_test! {
    get_agents_research,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_agents_research(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/agents_research",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_agents_research())
        .build();
    mock_response = serde_json::json!([{
        "agent_id": 100,
        "points_per_day": 1.07832178,
        "remainder_points": 1.07832178,
        "skill_type_id": 100,
        "started_at": "2018-12-20T16:11:54Z",
    }]),
}

authenticated_endpoint_test! {
    get_blueprints,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        let page = 0;
        esi_client
            .character()
            .get_blueprints(&access_token, character_id, page)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/blueprints?page=0",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_blueprints())
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

public_endpoint_test! {
    get_character_corporation_history,
    |esi_client: eve_esi::Client | async move {
        let character_ids = vec![2114794365, 2117053828];
        esi_client
            .character()
            .character_affiliation(character_ids)
            .await
    },
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
    url = "/characters/2114794365/cspa",
    mock_response = serde_json::json!([5000000])
}

authenticated_endpoint_test! {
    get_jump_fatigue,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_jump_fatigue(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/fatigue",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_fatigue())
        .build();
    mock_response = serde_json::json!({
        "jump_fatigue_expire_date": "2018-12-20T16:11:54Z",
        "last_jump_date": "2018-12-20T16:11:54Z",
        "last_update_date": "2018-12-20T16:11:54Z",
    }),
}

authenticated_endpoint_test! {
    get_medals,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_medals(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/medals",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_medals())
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
            "status": "Public",
            "title": "Medal name"
        }
    ]),
}

authenticated_endpoint_test! {
    get_character_notifications,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_character_notifications(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/notifications",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_notifications())
        .build();
    mock_response = serde_json::json!([
        {
            "is_read": false,
            "notification_id": 1,
            "sender": 2114794365,
            "sender_type": "Character",
            "text": "Notification text",
            "timestamp": "2018-12-20T16:11:54Z",
            "type": "AcceptedAlly"
        }
    ]),
}

public_endpoint_test! {
    get_character_portraits,
    |esi_client: eve_esi::Client | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_character_portraits(character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/portrait",
    mock_response = serde_json::json!({
        "px64x64": "ABCD",
        "px128x128": "ABCD",
        "px256x256": "ABCD",
        "px512x512": "ABCD",
    })
}

authenticated_endpoint_test! {
    read_corporation_roles,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .character()
            .get_character_corporation_roles(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/roles",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_corporation_roles())
        .build();
    mock_response = serde_json::json!([{
        "roles": ["Brand_Manager"],
        "roles_at_base": ["Brand_Manager"],
        "roles_at_hq": ["Brand_Manager"],
        "roles_at_other": ["Brand_Manager"],
    }]),
}
