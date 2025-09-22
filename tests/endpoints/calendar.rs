use eve_esi::{scope::CalendarScopes, ScopeBuilder};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_endpoint_test! {
    list_calendar_event_summaries,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .calendar()
            .list_calendar_event_summaries(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/calendar",
    required_scopes = ScopeBuilder::new()
        .calendar(CalendarScopes::new().read_calendar_events())
        .build();
    mock_response = serde_json::json!([
      {
        "event_date": "2019-08-24T14:15:22Z",
        "event_id": 0,
        "event_response": "declined",
        "importance": 0,
        "title": "string"
      }
    ]),
}
