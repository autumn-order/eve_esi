use eve_esi::scope::AlliancesScopes;
use eve_esi::ScopeBuilder;

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_endpoint_test! {
    get_alliance_contacts,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let alliance_id = 99013534;
        esi_client
            .contacts()
            .get_alliance_contacts(&access_token, alliance_id)
            .await
    },
    request_type = "GET",
    url = "/alliances/99013534/contacts",
    required_scopes = ScopeBuilder::new()
        .alliances(AlliancesScopes::new().read_contacts())
        .build();
    mock_response = serde_json::json!([
      {
        "contact_id": 0,
        "contact_type": "character",
        "label_ids": [
          0
        ],
        "standing": 0
      }
    ]),
}

authenticated_endpoint_test! {
    get_alliance_contact_labels,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let alliance_id = 99013534;
        esi_client
            .contacts()
            .get_alliance_contact_labels(&access_token, alliance_id)
            .await
    },
    request_type = "GET",
    url = "/alliances/99013534/contacts/labels",
    required_scopes = ScopeBuilder::new()
        .alliances(AlliancesScopes::new().read_contacts())
        .build();
    mock_response = serde_json::json!([
      {
        "label_id": 0,
        "label_name": "string"
      }
    ]),
}
