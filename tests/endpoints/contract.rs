use crate::util::integration_test_setup;

public_esi_request_test! {
    get_public_contracts,
    contracts,
    get_public_contracts[1, 1],
    request_type = "GET",
    url = "/contracts/public/1?page=1",
    mock_response = serde_json::json!(
        [
          {
            "buyout": 0,
            "collateral": 0,
            "contract_id": 0,
            "date_expired": "2019-08-24T14:15:22Z",
            "date_issued": "2019-08-24T14:15:22Z",
            "days_to_complete": 0,
            "end_location_id": 0,
            "for_corporation": true,
            "issuer_corporation_id": 0,
            "issuer_id": 0,
            "price": 0,
            "reward": 0,
            "start_location_id": 0,
            "title": "string",
            "type": "unknown",
            "volume": 0
          }
        ]
    )
}
