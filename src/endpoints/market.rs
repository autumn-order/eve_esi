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
//! ## Methods
//! - [`MarketEndpoints::list_open_orders_from_a_character`]: Fetches list of open market orders for the provided character ID
//! - [`MarketEndpoints::list_historical_orders_by_a_character]: Fetches list of cancelled & expired market orders for the provided character ID up to 90 days in the past

use crate::{
    model::market::{CharacterMarketOrder, CorporationMarketOrder, MarketItemGroupInformation},
    oauth2::scope::MarketScopes,
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
    /// For an overview & usage examples, see the [endpoints module documentation](super)
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
        /// - `Vec<MarketItemGroupInformation>`: The information of the provided market item group ID
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_item_group_information(
            market_group_id: i64
        ) -> Result<MarketItemGroupInformation, Error>
        url = "{}/markets/groups/{}";
        label = "market item group information";
    }
}
