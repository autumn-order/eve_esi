use eve_esi::{
    model::enums::calendar::PutCalendarEventResponse, scope::CalendarScopes, ScopeBuilder,
};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_esi_request_test! {
    list_calendar_event_summaries,
    calendar,
    list_calendar_event_summaries[2114794365],
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

authenticated_esi_request_test! {
    get_an_event,
    calendar,
    get_an_event[2114794365, 1],
    request_type = "GET",
    url = "/characters/2114794365/calendar/1",
    required_scopes = ScopeBuilder::new()
        .calendar(CalendarScopes::new().read_calendar_events())
        .build();
    mock_response = serde_json::json!({
      "date": "2019-08-24T14:15:22Z",
      "duration": 0,
      "event_id": 0,
      "importance": 0,
      "owner_id": 0,
      "owner_name": "string",
      "owner_type": "eve_server",
      "response": "string",
      "text": "string",
      "title": "string"
    }),
}

authenticated_esi_request_test! {
    respond_to_an_event,
    calendar,
    respond_to_an_event[2114794365, 1, PutCalendarEventResponse::Accepted],
    request_type = "PUT",
    url = "/characters/2114794365/calendar/1",
    required_scopes = ScopeBuilder::new()
        .calendar(CalendarScopes::new().respond_calendar_events())
        .build();
    mock_response = serde_json::json!(()),
}

authenticated_esi_request_test! {
    get_attendees,
    calendar,
    get_attendees[2114794365, 1],
    request_type = "GET",
    url = "/characters/2114794365/calendar/1/attendees",
    required_scopes = ScopeBuilder::new()
        .calendar(CalendarScopes::new().read_calendar_events())
        .build();
    mock_response = serde_json::json!([
      {
        "character_id": 0,
        "event_response": "declined"
      }
    ]),
}
