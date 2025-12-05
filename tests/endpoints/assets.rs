use eve_esi::{scope::AssetsScopes, ScopeBuilder};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_esi_request_test! {
    get_character_assets,
    assets,
    get_character_assets[2114794365, 1],
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

authenticated_esi_request_test! {
    get_character_asset_locations,
    assets,
    get_character_asset_locations[2114794365, vec![0]],
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

authenticated_esi_request_test! {
    get_character_asset_names,
    assets,
    get_character_asset_names[2114794365, vec![0]],
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

authenticated_esi_request_test! {
    get_corporation_assets,
    assets,
    get_corporation_assets[98785281, 1],
    request_type = "GET",
    url = "/corporations/98785281/assets?page=1",
    required_scopes = ScopeBuilder::new()
        .assets(AssetsScopes::new().read_corporation_assets())
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

authenticated_esi_request_test! {
    get_corporation_asset_locations,
    assets,
    get_corporation_asset_locations[98785281, vec![0]],
    request_type = "POST",
    url = "/corporations/98785281/assets/locations",
    required_scopes = ScopeBuilder::new()
        .assets(AssetsScopes::new().read_corporation_assets())
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

authenticated_esi_request_test! {
    get_corporation_asset_names,
    assets,
    get_corporation_asset_names[98785281, vec![0]],
    request_type = "POST",
    url = "/corporations/98785281/assets/names",
    required_scopes = ScopeBuilder::new()
        .assets(AssetsScopes::new().read_corporation_assets())
        .build();
    mock_response = serde_json::json!([
      {
        "item_id": 0,
        "name": "string"
      }
    ]),
}
