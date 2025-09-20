use eve_esi::{oauth2::scope::MarketScopes, ScopeBuilder};

use crate::endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes};

authenticated_endpoint_test! {
    list_open_orders_from_a_character,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        esi_client
            .market()
            .list_open_orders_from_a_character(&access_token, character_id)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/orders",
    required_scopes = ScopeBuilder::new()
        .market(MarketScopes::new().read_character_orders())
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

authenticated_endpoint_test! {
    list_historical_orders_by_a_character,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let character_id = 2114794365;
        let page = 1;
        esi_client
            .market()
            .list_historical_orders_by_a_character(&access_token, character_id, page)
            .await
    },
    request_type = "GET",
    url = "/characters/2114794365/orders/history?page=1",
    required_scopes = ScopeBuilder::new()
        .market(MarketScopes::new().read_character_orders())
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

authenticated_endpoint_test! {
    list_open_orders_from_a_corporation,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        esi_client
            .market()
            .list_open_orders_from_a_corporation(&access_token, corporation_id)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/orders",
    required_scopes = ScopeBuilder::new()
        .market(MarketScopes::new().read_corporation_orders())
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

authenticated_endpoint_test! {
    list_historical_orders_from_a_corporation,
    |esi_client: eve_esi::Client, access_token: String | async move {
        let corporation_id = 98785281;
        let page = 1;
        esi_client
            .market()
            .list_historical_orders_from_a_corporation(&access_token, corporation_id, page)
            .await
    },
    request_type = "GET",
    url = "/corporations/98785281/orders/history?page=1",
    required_scopes = ScopeBuilder::new()
        .market(MarketScopes::new().read_corporation_orders())
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
