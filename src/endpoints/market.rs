//! # EVE ESI Market Endpoints
//!
//! This module provides the [`MarketEndpoints`] struct and associated methods for accessing
//! market-related ESI endpoints.

use crate::{
    esi::EsiRequest,
    model::{
        enums::market::OrderType,
        market::{
            CharacterMarketOrder, CorporationMarketOrder, MarketItemGroupInformation,
            MarketItemPrices, MarketItemRegionStatistics, MarketRegionOrder, StructureMarketOrder,
        },
    },
    scope::MarketsScopes,
    Client, ScopeBuilder,
};
use reqwest::Method;

/// Provides methods for accessing market-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct MarketEndpoints<'a> {
    client: &'a Client,
}

impl<'a> MarketEndpoints<'a> {
    /// Creates a new instance of [`MarketEndpoints`].
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_esi_endpoint! {
        /// Fetches list of open market orders for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdOrders>
        ///
        /// # Required Scopes
        /// - [`MarketsScopes::read_character_orders`](crate::scope::MarketsScopes::read_character_orders):
        ///   `esi-markets.read_character_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`    (`i64`): The ID of the character to retrieve open market orders for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of open market orders for the provided character ID when sent.
        auth fn list_open_orders_from_a_character(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<CharacterMarketOrder>>
        method = Method::GET;
        path = "/characters/{}/orders";
        required_scopes = ScopeBuilder::new()
            .markets(MarketsScopes::new().read_character_orders())
            .build();
    }

    define_esi_endpoint! {
        /// Fetches list of cancelled & expired market orders for the provided character ID up to 90 days in the past
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdOrdersHistory>
        ///
        /// # Required Scopes
        /// - [`MarketsScopes::read_character_orders`](crate::scope::MarketsScopes::read_character_orders):
        ///   `esi-markets.read_character_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`    (`i64`): The ID of the character to retrieve historical market orders for
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a list of historical market orders for the provided character ID when sent.
        auth fn list_historical_orders_by_a_character(
            access_token: &str,
            character_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CharacterMarketOrder>>
        method = Method::GET;
        path = "/characters/{}/orders/history";
        required_scopes = ScopeBuilder::new()
            .markets(MarketsScopes::new().read_character_orders())
            .build();
    }

    define_esi_endpoint! {
        /// Fetches list of open market orders for the provided corporation ID
        ///
        /// Additional permissions required: the owner of the access token must hold the `Accountant` or
        /// `Trader` role within the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdOrders>
        ///
        /// # Required Scopes
        /// - [`MarketsScopes::read_corporation_orders`](crate::scope::MarketsScopes::read_corporation_orders):
        ///   `esi-markets.read_corporation_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve open market orders for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of open market orders for the provided corporation ID when sent.
        auth fn list_open_orders_from_a_corporation(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationMarketOrder>>
        method = Method::GET;
        path = "/corporations/{}/orders";
        required_scopes = ScopeBuilder::new()
            .markets(MarketsScopes::new().read_corporation_orders())
            .build();
    }

    define_esi_endpoint! {
        /// Fetches list of cancelled & expired market orders for the provided corporation ID up to 90 days in the past
        ///
        /// Additional permissions required: the owner of the access token must hold the `Accountant` or
        /// `Trader` role within the corporation to access this information.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdOrdersHistory>
        ///
        /// # Required Scopes
        /// - [`MarketsScopes::read_corporation_orders`](crate::scope::MarketsScopes::read_corporation_orders):
        ///   `esi-markets.read_corporation_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve historical market orders for
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a list of historical market orders for the provided corporation ID when sent.
        auth fn list_historical_orders_from_a_corporation(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> EsiRequest<Vec<CorporationMarketOrder>>
        method = Method::GET;
        path = "/corporations/{}/orders/history";
        required_scopes = ScopeBuilder::new()
            .markets(MarketsScopes::new().read_corporation_orders())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves a list of IDs of market item groups
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsGroups>
        ///
        /// # Returns
        /// An ESI request builder that returns a list of IDs of market item groups when sent.
        pub fn get_item_groups(
        ) -> EsiRequest<Vec<i64>>
        method = Method::GET;
        path = "/markets/groups";
    }

    define_esi_endpoint! {
        /// Retrieves the information of the provided market item group ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsGroupsMarketGroupId>
        ///
        /// # Arguments
        /// - `market_group_id` (`i64`): The ID of the market group to retrieve information for.
        ///
        /// # Returns
        /// An ESI request builder that returns the information of the provided market item group ID when sent.
        pub fn get_item_group_information(
            market_group_id: i64
        ) -> EsiRequest<MarketItemGroupInformation>
        method = Method::GET;
        path = "/markets/groups/{}";
    }

    define_esi_endpoint! {
        /// Retrieves the average & adjusted market prices of all items
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsPrices>
        ///
        /// # Returns
        /// An ESI request builder that returns the average & adjusted market prices of all items when sent.
        pub fn list_market_prices(
        ) -> EsiRequest<Vec<MarketItemPrices>>
        method = Method::GET;
        path = "/markets/prices";
    }

    define_esi_endpoint! {
        /// Fetches list of market orders for the provided structure ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsStructuresStructureId>
        ///
        /// # Required Scopes
        /// - [`MarketsScopes::structure_markets`](crate::scope::MarketsScopes::structure_markets):
        ///   `esi-markets.structure_markets.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `structure_id`    (`i64`): The ID of the structure to retrieve market orders for
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a list of market orders for the provided structure ID when sent.
        auth fn list_orders_in_a_structure(
            access_token: &str,
            structure_id: i64;
            page: i32
        ) -> EsiRequest<Vec<StructureMarketOrder>>
        method = Method::GET;
        path = "/markets/structures/{}";
        required_scopes = ScopeBuilder::new()
            .markets(MarketsScopes::new().structure_markets())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves list of entries with historical market statistics for the provided item type ID in provided region ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsRegionIdHistory>
        ///
        /// # Arguments
        /// - `region_id` (`i64`): ID of the region to retrieve market statistics for the specified item type ID
        /// - `type_id`   (`i64`): ID of the item type to retrieve market statistics for in the specified region ID
        ///
        /// # Returns
        /// An ESI request builder that returns a list of entries with historical market statistics for the provided item type ID in provided region ID when sent.
        pub fn list_historical_market_statistics_in_a_region(
            region_id: i64;
            type_id: i64
        ) -> EsiRequest<Vec<MarketItemRegionStatistics>>
        method = Method::GET;
        path = "/markets/{}/history";
    }

    define_esi_endpoint! {
        /// Retrieves a list of market orders within the provided region ID and of the specified order type
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsRegionIdOrders>
        ///
        /// # Arguments
        /// - `region_id`   (`i64`): ID of the region to retrieve market orders for
        /// - `order_type`  ([`OrderType`]): Enum representing type of market order to request, either [`OrderType::Sell`],
        ///   [`OrderType::Buy`], or [`OrderType::All`] for both
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a list of market orders within the provided region ID and of the specified order type when sent.
        pub fn list_orders_in_a_region(
            region_id: i64;
            order_type: OrderType,
            page: i32
        ) -> EsiRequest<Vec<MarketRegionOrder>>
        method = Method::GET;
        path = "/markets/{}/orders";
    }

    define_esi_endpoint! {
        /// Retrieves a list of type IDs that have active market orders for the given region ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsRegionIdTypes>
        ///
        /// # Arguments
        /// - `region_id`   (`i64`): ID of the region to retrieve item type IDs for
        /// - `page`        (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a list of type IDs that have active market orders for the given region ID when sent.
        pub fn list_type_ids_relevant_to_a_market(
            region_id: i64;
            page: i32
        ) -> EsiRequest<Vec<i64>>
        method = Method::GET;
        path = "/markets/{}/types";
    }
}
