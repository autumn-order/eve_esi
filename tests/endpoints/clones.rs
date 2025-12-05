use eve_esi::scope::ClonesScopes;
use eve_esi::ScopeBuilder;

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_esi_request_test! {
    get_clones,
    clones,
    get_clones[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/clones",
    required_scopes = ScopeBuilder::new()
        .clones(ClonesScopes::new().read_clones())
        .build();
    mock_response = serde_json::json!({
      "home_location": {
        "location_id": 0,
        "location_type": "station"
      },
      "jump_clones": [
        {
          "implants": [
            null
          ],
          "jump_clone_id": 0,
          "location_id": 0,
          "location_type": "station",
          "name": "string"
        }
      ],
      "last_clone_jump_date": "2019-08-24T14:15:22Z",
      "last_station_change_date": "2019-08-24T14:15:22Z"
    }),
}

authenticated_esi_request_test! {
    get_active_implants,
    clones,
    get_active_implants[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/implants",
    required_scopes = ScopeBuilder::new()
        .clones(ClonesScopes::new().read_implants())
        .build();
    mock_response = serde_json::json!([
      0
    ]),
}
