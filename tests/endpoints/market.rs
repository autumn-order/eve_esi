use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};
use crate::util::integration_test_setup;
use eve_esi::model::enums::market::OrderType;
use eve_esi::{scope::MarketsScopes, ScopeBuilder};

authenticated_esi_request_test! {
    list_open_orders_from_a_character,
    market,
    list_open_orders_from_a_character[2114794365],
    request_type = "GET",
    url = "/characters/2114794365/orders",
    required_scopes = ScopeBuilder::new()
        .markets(MarketsScopes::new().read_character_orders())
        .build();
    mock_response = serde_json::json!([
      {
        "duration": 0,
        "escrow": 0,
        "is_buy_order": true,
        "is_corporation": true,
        "issued": "2019-08-24T14:15:22Z",
        "location_id": 0,
        "min_volume": 0,
        "order_id": 0,
        "price": 0,
        "range": "1",
        "region_id": 0,
        "type_id": 0,
        "volume_remain": 0,
        "volume_total": 0
      }
    ]),
}

authenticated_esi_request_test! {
    list_historical_orders_by_a_character,
    market,
    list_historical_orders_by_a_character[2114794365, 1],
    request_type = "GET",
    url = "/characters/2114794365/orders/history?page=1",
    required_scopes = ScopeBuilder::new()
        .markets(MarketsScopes::new().read_character_orders())
        .build();
    mock_response = serde_json::json!([
      {
        "duration": 0,
        "escrow": 0,
        "is_buy_order": true,
        "is_corporation": true,
        "issued": "2019-08-24T14:15:22Z",
        "location_id": 0,
        "min_volume": 0,
        "order_id": 0,
        "price": 0,
        "range": "1",
        "region_id": 0,
        "state": "cancelled",
        "type_id": 0,
        "volume_remain": 0,
        "volume_total": 0
      }
    ]),
}

authenticated_esi_request_test! {
    list_open_orders_from_a_corporation,
    market,
    list_open_orders_from_a_corporation[98785281],
    request_type = "GET",
    url = "/corporations/98785281/orders",
    required_scopes = ScopeBuilder::new()
        .markets(MarketsScopes::new().read_corporation_orders())
        .build();
    mock_response = serde_json::json!([
      {
        "duration": 0,
        "escrow": 0,
        "is_buy_order": true,
        "issued": "2019-08-24T14:15:22Z",
        "issued_by": 0,
        "location_id": 0,
        "min_volume": 0,
        "order_id": 0,
        "price": 0,
        "range": "1",
        "region_id": 0,
        "type_id": 0,
        "volume_remain": 0,
        "volume_total": 0,
        "wallet_division": 0
      }
    ]),
}

authenticated_esi_request_test! {
    list_historical_orders_from_a_corporation,
    market,
    list_historical_orders_from_a_corporation[98785281, 1],
    request_type = "GET",
    url = "/corporations/98785281/orders/history?page=1",
    required_scopes = ScopeBuilder::new()
        .markets(MarketsScopes::new().read_corporation_orders())
        .build();
    mock_response = serde_json::json!([
      {
        "duration": 0,
        "escrow": 0,
        "is_buy_order": true,
        "issued": "2019-08-24T14:15:22Z",
        "issued_by": 0,
        "location_id": 0,
        "min_volume": 0,
        "order_id": 0,
        "price": 0,
        "range": "1",
        "region_id": 0,
        "state": "cancelled",
        "type_id": 0,
        "volume_remain": 0,
        "volume_total": 0,
        "wallet_division": 0
      }
    ]),
}

public_esi_request_test! {
    get_item_groups,
    market,
    get_item_groups[],
    request_type = "GET",
    url = "/markets/groups",
    mock_response = serde_json::json!([
      0
    ])
}

public_esi_request_test! {
    get_item_group_information,
    market,
    get_item_group_information[1],
    request_type = "GET",
    url = "/markets/groups/1",
    mock_response = serde_json::json!({
      "description": "string",
      "market_group_id": 0,
      "name": "string",
      "parent_group_id": 0,
      "types": [
        0
      ]
    })
}

public_esi_request_test! {
    list_market_prices,
    market,
    list_market_prices[],
    request_type = "GET",
    url = "/markets/prices",
    mock_response = serde_json::json!([
      {
        "adjusted_price": 0,
        "average_price": 0,
        "type_id": 0
      }
    ])
}

authenticated_esi_request_test! {
    list_orders_in_a_structure,
    market,
    list_orders_in_a_structure[1, 1],
    request_type = "GET",
    url = "/markets/structures/1?page=1",
    required_scopes = ScopeBuilder::new()
        .markets(MarketsScopes::new().structure_markets())
        .build();
    mock_response = serde_json::json!([
      {
        "duration": 0,
        "is_buy_order": true,
        "issued": "2019-08-24T14:15:22Z",
        "location_id": 0,
        "min_volume": 0,
        "order_id": 0,
        "price": 0,
        "range": "station",
        "type_id": 0,
        "volume_remain": 0,
        "volume_total": 0
      }
    ]),
}

public_esi_request_test! {
    list_historical_market_statistics_in_a_region,
    market,
    list_historical_market_statistics_in_a_region[1, 1],
    request_type = "GET",
    url = "/markets/1/history?type_id=1",
    mock_response = serde_json::json!([
      {
        "average": 0,
        "date": "2019-08-24",
        "highest": 0,
        "lowest": 0,
        "order_count": 0,
        "volume": 0
      }
    ])
}

public_esi_request_test! {
    list_orders_in_a_region,
    market,
    list_orders_in_a_region[1, OrderType::All, 1],
    request_type = "GET",
    url = "/markets/1/orders?order_type=%22all%22&page=1",
    mock_response = serde_json::json!([
      {
        "duration": 0,
        "is_buy_order": true,
        "issued": "2019-08-24T14:15:22Z",
        "location_id": 0,
        "min_volume": 0,
        "order_id": 0,
        "price": 0,
        "range": "station",
        "system_id": 0,
        "type_id": 0,
        "volume_remain": 0,
        "volume_total": 0
      }
    ])
}

public_esi_request_test! {
    list_type_ids_relevant_to_a_market,
    market,
    list_type_ids_relevant_to_a_market[1, 1],
    request_type = "GET",
    url = "/markets/1/types?page=1",
    mock_response = serde_json::json!([0])
}
