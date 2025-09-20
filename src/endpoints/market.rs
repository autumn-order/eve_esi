//! # EVE ESI Market Endpoints
//!
//! This module provides the [`MarketEndpoints`] struct and associated methods for accessing
//! market-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (11)
//! ### Public (6)
//! - [`MarketEndpoints::get_item_groups`]: Retrieves a list of IDs of market item groups
//! - [`MarketEndpoints::get_item_group_information`]: Retrieves the information of the provided market item group ID
//! - [`MarketEndpoints::list_market_prices`]: Retrieves the average & adjusted market prices of all items
//! - [`MarketEndpoints::list_historical_market_statistics_in_a_region`]: List of entries with historical market statistics for the provided item type ID in provided region ID
//! - [`MarketEndpoints::list_orders_in_a_region]: Retrieves a list of market orders within the provided region ID and of the specified order type
//! - [`MarketEndpoints::list_type_ids_relevant_to_a_market`]: Retrieves a list of type IDs that have active market orders for the given region ID
//!
//! ### Authenticated (5)
//! - [`MarketEndpoints::list_open_orders_from_a_character`]: Fetches list of open market orders for the provided character ID
//! - [`MarketEndpoints::list_historical_orders_by_a_character`]: Fetches list of cancelled & expired market orders for the provided character ID up to 90 days in the past
//! - [`MarketEndpoints::list_open_orders_from_a_corporation`]: Fetches list of open market orders for the provided corporation ID
//! - [`MarketEndpoints::list_historical_orders_from_a_corporation`]: Fetches list of cancelled & expired market orders for the provided corporation ID up to 90 days in the past
//! - [`MarketEndpoints::list_orders_in_a_structure`]: Fetches list of market orders for the provided structure ID

use crate::{
    model::{
        enums::market::OrderType,
        market::{
            CharacterMarketOrder, CorporationMarketOrder, MarketItemGroupInformation,
            MarketItemPrices, MarketItemRegionStatistics, MarketRegionOrder, StructureMarketOrder,
        },
    },
    scope::MarketScopes,
    Client, Error, ScopeBuilder,
};

/// Provides methods for accessing market-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct MarketEndpoints<'a> {
    client: &'a Client,
}

impl<'a> MarketEndpoints<'a> {
    /// Creates a new instance of [`MarketEndpoints`].
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)e
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Fetches list of open market orders for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdOrders>
        ///
        /// # Required Scopes
        /// - [`MarketScopes::read_character_orders`](crate::oauth2::scope::MarketScopes::read_character_orders):
        ///   `esi-markets.read_character_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`    (`i64`): The ID of the character to retrieve open market orders for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterMarketOrder`]`>`: List of open market orders for the provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get list_open_orders_from_a_character(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterMarketOrder>, Error>
        url = "{}/characters/{}/orders";
        label = "open market orders";
        required_scopes = ScopeBuilder::new().market(MarketScopes::new().read_character_orders()).build();
    }

    define_endpoint! {
        /// Fetches list of cancelled & expired market orders for the provided character ID up to 90 days in the past
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdOrdersHistory>
        ///
        /// # Required Scopes
        /// - [`MarketScopes::read_character_orders`](crate::oauth2::scope::MarketScopes::read_character_orders):
        ///   `esi-markets.read_character_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`    (`i64`): The ID of the character to retrieve historical market orders for
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterHistoricalMarketOrder`]`>`: List of historical market orders for the provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get list_historical_orders_by_a_character(
            access_token: &str,
            character_id: i64,
            page: i32,
        ) -> Result<Vec<CharacterMarketOrder>, Error>
        url = "{}/characters/{}/orders/history?page={}";
        label = "historical orders";
        required_scopes = ScopeBuilder::new().market(MarketScopes::new().read_character_orders()).build();
    }

    define_endpoint! {
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
        /// - [`MarketScopes::read_corporation_orders`](crate::oauth2::scope::MarketScopes::read_corporation_orders):
        ///   `esi-markets.read_corporation_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve open market orders for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationMarketOrder`]`>`: List of open market orders for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get list_open_orders_from_a_corporation(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<CorporationMarketOrder>, Error>
        url = "{}/corporations/{}/orders";
        label = "open orders";
        required_scopes = ScopeBuilder::new().market(MarketScopes::new().read_corporation_orders()).build();
    }

    define_endpoint! {
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
        /// - [`MarketScopes::read_corporation_orders`](crate::oauth2::scope::MarketScopes::read_corporation_orders):
        ///   `esi-markets.read_corporation_orders.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve historical market orders for
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationHistoricalMarketOrder`]`>`: List of historical market orders for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get list_historical_orders_from_a_corporation(
            access_token: &str,
            corporation_d: i64,
            page: i32,
        ) -> Result<Vec<CorporationMarketOrder>, Error>
        url = "{}/corporations/{}/orders/history?page={}";
        label = "historical orders";
        required_scopes = ScopeBuilder::new().market(MarketScopes::new().read_corporation_orders()).build();
    }

    define_endpoint! {
        /// Retrieves a list of IDs of market item groups
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsGroups>
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: List of IDs of market item groups
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_item_groups(
        ) -> Result<Vec<i64>, Error>
        url = "{}/markets/groups";
        label = "market groups";
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`MarketItemGroupInformation`]`>`: The information of the provided market item group ID
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_item_group_information(
            market_group_id: i64
        ) -> Result<MarketItemGroupInformation, Error>
        url = "{}/markets/groups/{}";
        label = "market item group information";
    }

    define_endpoint! {
        /// Retrieves the average & adjusted market prices of all items
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsPrices>
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`MarketItemPrices`]`>`: The average & adjusted market prices of all items
        /// - [`Error`]: An error if the fetch request fails
        pub_get list_market_prices(
        ) -> Result<Vec<MarketItemPrices>, Error>
        url = "{}/markets/prices";
        label = "market item prices";
    }

    define_endpoint! {
        /// Fetches list of market orders for the provided structure ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdOrdersHistory>
        ///
        /// # Required Scopes
        /// - [`MarketScopes::structure_markets`](crate::oauth2::scope::MarketScopes::structure_markets):
        ///   `esi-markets.structure_markets.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the structure to retrieve market orders for
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`StructureMarketOrder`]`>`: List of market orders for the provided structure ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get list_orders_in_a_structure(
            access_token: &str,
            structure_id: i64,
            page: i32,
        ) -> Result<Vec<StructureMarketOrder>, Error>
        url = "{}/markets/structures/{}?page={}";
        label = "market orders";
        required_scopes = ScopeBuilder::new().market(MarketScopes::new().structure_markets()).build();
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`MarketItemRegionStatistics`]`>`: List of entries with historical market statistics for the provided item type ID in provided region ID
        /// - [`Error`]: An error if the fetch request fails
        pub_get list_historical_market_statistics_in_a_region(
            region_id: i64,
            type_id: i64
        ) -> Result<Vec<MarketItemRegionStatistics>, Error>
        url = "{}/markets/{}/history?type_id={}";
        label = "regional market statistics for item";
    }

    define_endpoint! {
        /// Retrieves a list of market orders within the provided region ID and of the specified order type
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsRegionIdHistory>
        ///
        /// # Arguments
        /// - `region_id`   (`i64`): ID of the region to retrieve market orders for
        /// - `order_type`  ([`OrderType`]): Enum representing type of market order to request, either [`OrderType::Sell`],
        ///   [`OrderType::Buy`], or [`OrderType::All`] for both
        /// - `page`            (`i32`): The page of market orders to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`MarketRegionOrder`]`>`: list of market orders within the provided region ID and of the specified order type
        pub_get list_orders_in_a_region(
            region_id: i64,
            order_type: OrderType,
            page: i32
        ) -> Result<Vec<MarketRegionOrder>, Error>
        url = "{}/markets/{}/orders?order_type={}&page={}";
        label = "market orders";
    }

    define_endpoint! {
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
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: list of type IDs that have active market orders for the given region ID
        pub_get list_type_ids_relevant_to_a_market(
            region_id: i64,
            page: i32
        ) -> Result<Vec<i64>, Error>
        url = "{}/markets/{}/types?page={}";
        label = "item type IDs with active market orders";
    }
}
