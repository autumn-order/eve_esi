use eve_esi::{scope::AssetsScopes, ScopeBuilder};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_endpoint_test! {
    get_character_assets,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        let page = 1;
        esi_client
            .assets()
            .get_character_assets(&access_token, character_id, page)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/assets?page=1",
    required_scopes = ScopeBuilder::new()
        .assets(AssetsScopes::new().read_assets())
        .build();
    mock_response = serde_json::json!([
      {
        "is_blueprint_copy": true,
        "is_singleton": true,
        "item_id": 0,
        "location_flag": "AssetSafety",
        "location_id": 0,
        "location_type": "station",
        "quantity": 0,
        "type_id": 0
      }
    ]),
}

authenticated_endpoint_test! {
    get_character_asset_locations,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        let item_ids = vec![0];
        esi_client
            .assets()
            .get_character_asset_locations(&access_token, item_ids, character_id)
            .await
    },
    request_type = "POST",
    url = "/characters/2114794365/assets/locations",
    required_scopes = ScopeBuilder::new()
        .assets(AssetsScopes::new().read_assets())
        .build();
    mock_response = serde_json::json!([
      {
        "item_id": 0,
        "position": {
          "x": 0,
          "y": 0,
          "z": 0
        }
      }
    ]),
}

authenticated_endpoint_test! {
    get_character_asset_names,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        let item_ids = vec![0];
        esi_client
            .assets()
            .get_character_asset_names(&access_token, item_ids, character_id)
            .await
    },
    request_type = "POST",
    url = "/characters/2114794365/assets/names",
    required_scopes = ScopeBuilder::new()
        .assets(AssetsScopes::new().read_assets())
        .build();
    mock_response = serde_json::json!([
      {
        "item_id": 0,
        "name": "string"
      }
    ]),
}
