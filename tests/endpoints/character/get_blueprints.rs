use eve_esi::{oauth2::scope::CharacterScopes, ScopeBuilder};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

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
    url = "/characters/2114794365/blueprints?page=0",
    required_scopes = ScopeBuilder::new()
        .character(CharacterScopes::new().read_blueprints())
        .build();
}
