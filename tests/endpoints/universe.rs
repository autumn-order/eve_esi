use crate::util::integration_test_setup;

public_esi_request_test! {
    get_factions,
    |esi_client: eve_esi::Client | {
        esi_client
            .universe()
            .get_factions()
    },
    request_type = "GET",
    url = "/universe/factions",
    mock_response = serde_json::json!([
      {
        "corporation_id": 0,
        "description": "string",
        "faction_id": 0,
        "is_unique": true,
        "militia_corporation_id": 0,
        "name": "string",
        "size_factor": 0,
        "solar_system_id": 0,
        "station_count": 0,
        "station_system_count": 0
      }
    ])
}
