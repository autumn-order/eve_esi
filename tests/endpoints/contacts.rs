use eve_esi::scope::{AlliancesScopes, CharactersScopes};
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

authenticated_endpoint_test! {
    delete_contacts,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        let contact_ids = vec![1];
        esi_client
            .contacts()
            .delete_contacts(&access_token, contact_ids, character_id)
            .await
    },
    request_type = "DELETE",
    url = "/characters/2114794365/contacts?contact_ids=[1]",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().write_contacts())
        .build();
    mock_response = serde_json::json!(()),
}

authenticated_endpoint_test! {
    get_contacts,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .contacts()
            .get_contacts(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/contacts",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_contacts())
        .build();
    mock_response = serde_json::json!([
      {
        "contact_id": 0,
        "contact_type": "character",
        "is_blocked": true,
        "is_watched": true,
        "label_ids": [
          0
        ],
        "standing": 0
      }
    ]),
}
