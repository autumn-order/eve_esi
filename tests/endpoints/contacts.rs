use eve_esi::scope::{AlliancesScopes, CharactersScopes, CorporationsScopes};
use eve_esi::ScopeBuilder;

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_esi_request_test! {
    get_alliance_contacts,
    |esi_client: eve_esi::Client, access_token: String | {
        let alliance_id = 99013534;
        esi_client
            .contacts()
            .get_alliance_contacts(&access_token, alliance_id)
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

authenticated_esi_request_test! {
    get_alliance_contact_labels,
    |esi_client: eve_esi::Client, access_token: String | {
        let alliance_id = 99013534;
        esi_client
            .contacts()
            .get_alliance_contact_labels(&access_token, alliance_id)
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

authenticated_esi_request_test! {
    delete_contacts,
    |esi_client: eve_esi::Client, access_token: String | {
        let character_id = 2114794365;
        let contact_ids = vec![1,2,3];
        esi_client
            .contacts()
            .delete_contacts(&access_token, character_id, contact_ids)
    },
    request_type = "DELETE",
    // Note: contact_ids array is percent encoded due to usage of URL serializer
    url = "/characters/2114794365/contacts?contact_ids=%5B1%2C2%2C3%5D",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().write_contacts())
        .build();
    mock_response = serde_json::json!(()),
}

authenticated_esi_request_test! {
    get_contacts,
    |esi_client: eve_esi::Client, access_token: String | {
        let character_id = 2114794365;
        esi_client
            .contacts()
            .get_contacts(&access_token, character_id)
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

authenticated_esi_request_test! {
    add_contacts,
    |esi_client: eve_esi::Client, access_token: String | {
        let character_id = 2114794365;
        let standing = -10.0;
        let label_ids = vec![1,2,3];
        let watched = false;
        let contact_ids = vec![1,2,3];
        esi_client
            .contacts()
            .add_contacts(&access_token, character_id, standing, label_ids, watched, contact_ids)
    },
    request_type = "POST",
    // Note: label_ids array is percent encoded due to usage of URL serializer
    url = "/characters/2114794365/contacts?standing=-10.0&label_ids=%5B1%2C2%2C3%5D&watched=false",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().write_contacts())
        .build();
    mock_response = serde_json::json!([1,2,3]),
}

authenticated_esi_request_test! {
    edit_contacts,
    |esi_client: eve_esi::Client, access_token: String | {
        let character_id = 2114794365;
        let standing = -10.0;
        let label_ids = vec![1,2,3];
        let watched = false;
        let contact_ids = vec![1,2,3];
        esi_client
            .contacts()
            .edit_contacts(&access_token, character_id, standing, label_ids, watched, contact_ids)
    },
    request_type = "PUT",
    // Note: label_ids array is percent encoded due to usage of URL serializer
    url = "/characters/2114794365/contacts?standing=-10.0&label_ids=%5B1%2C2%2C3%5D&watched=false",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().write_contacts())
        .build();
    mock_response = serde_json::json!([1,2,3]),
}

authenticated_esi_request_test! {
    get_contact_labels,
    |esi_client: eve_esi::Client, access_token: String | {
        let character_id = 2114794365;
        esi_client
            .contacts()
            .get_contact_labels(&access_token, character_id)
    },
    request_type = "GET",
    url = "/characters/2114794365/contacts/labels",
    required_scopes = ScopeBuilder::new()
        .characters(CharactersScopes::new().read_contacts())
        .build();
    mock_response = serde_json::json!([
      {
        "label_id": 0,
        "label_name": "string"
      }
    ]),
}

authenticated_esi_request_test! {
    get_corporation_contacts,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .contacts()
            .get_corporation_contacts(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/contacts",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_contacts())
        .build();
    mock_response = serde_json::json!([
      {
        "contact_id": 0,
        "contact_type": "character",
        "is_watched": true,
        "label_ids": [
          0
        ],
        "standing": 0
      }
    ]),
}

authenticated_esi_request_test! {
    get_corporation_contact_labels,
    |esi_client: eve_esi::Client, access_token: String | {
        let corporation_id = 98785281;
        esi_client
            .contacts()
            .get_corporation_contact_labels(&access_token, corporation_id)
    },
    request_type = "GET",
    url = "/corporations/98785281/contacts/labels",
    required_scopes = ScopeBuilder::new()
        .corporations(CorporationsScopes::new().read_contacts())
        .build();
    mock_response = serde_json::json!([
      {
        "label_id": 0,
        "label_name": "string"
      }
    ]),
}
