use eve_esi::{oauth2::scope::CharacterScopes, ScopeBuilder};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

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
    mock_response = serde_json::json!([{
        "agent_id": 100,
        "points_per_day": 1.07832178,
        "remainder_points": 1.07832178,
        "skill_type_id": 100,
        "started_at": "2018-12-20T16:11:54Z",
    }]),
    url = "/characters/2114794365/agents_research",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_agents_research())
        .build();
}
