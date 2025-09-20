//! # EVE ESI Market Models
//!
//! Provides models related to market endpoints for EVE Online's ESI API.
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Models
//! - [`CharacterMarketOrder`]: Details for a character's market order
//! - [`CorporationMarketOrder`]: Details for a corporation's market order
//! - [`MarketItemGroupInformation`]: Information regarding a specific market group
//! - [`MarketItemPrices`]: The average & adjusted market prices of an item
//! - [`StructureMarketOrder`]: Details for a market order placed within a structure
//! - [`MarketItemRegionStatistics`]: An entry for the market statistics of an item within a specific region on a given date

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::market::{HistoricalMarketOrderState, MarketOrderRange};

/// Details for a character's market order
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdOrdersGet>
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdOrdersHistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterMarketOrder {
    /// Number of days for which the order is valid
    /// starting from the issued date.
    ///
    /// An order expires at time issued + duration
    pub duration: i64,
    /// For buy orders, the amount of ISK in escrow
    pub escrow: Option<f64>,
    /// True if the order is a buy order
    #[serde(default)]
    pub is_buy_order: bool,
    /// Indicates whether or not order was placed on behalf of a corporation
    pub is_corporation: bool,
    /// Date and time when the order was issued
    pub issued: DateTime<Utc>,
    /// ID of the location where order was placed
    pub location_id: i64,
    /// For buy orders, the minimum quantity that will be accepted in a matching sell order
    pub min_volume: Option<i64>,
    /// Unique ID of the order
    pub order_id: i64,
    /// The cost per unit for this order
    pub price: f64,
    /// The range of the order
    pub range: MarketOrderRange,
    /// ID of the region where the order was placed
    pub region_id: i64,
    /// If it is a historical market order, indicates whether it was cancelled or expired
    pub state: Option<HistoricalMarketOrderState>,
    /// The type ID of the item in the order
    pub type_id: i64,
    /// Remaining quantity of items still for sale or buy
    pub volume_remain: i64,
    /// Quantity of items for sale or to buy when the order was placed
    pub volume_total: i64,
}

/// Details for a corporation's market order
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdOrdersGet>
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdOrdersHistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CorporationMarketOrder {
    /// Number of days for which the order is valid
    /// starting from the issued date.
    ///
    /// An order expires at time issued + duration
    pub duration: i64,
    /// For buy orders, the amount of ISK in escrow
    pub escrow: Option<f64>,
    /// True if the order is a buy order
    #[serde(default)]
    pub is_buy_order: bool,
    /// Date and time when the order was issued
    pub issued: DateTime<Utc>,
    /// Character ID of who issued the market order
    pub issued_by: i64,
    /// ID of the location where order was placed
    pub location_id: i64,
    /// For buy orders, the minimum quantity that will be accepted in a matching sell order
    pub min_volume: Option<i64>,
    /// Unique ID of the order
    pub order_id: i64,
    /// The cost per unit for this order
    pub price: f64,
    /// The range of the order
    pub range: MarketOrderRange,
    /// ID of the region where the order was placed
    pub region_id: i64,
    /// If it is a historical market order, indicates whether it was cancelled or expired
    pub state: Option<HistoricalMarketOrderState>,
    /// The type ID of the item in the order
    pub type_id: i64,
    /// Remaining quantity of items still for sale or buy
    pub volume_remain: i64,
    /// Quantity of items for sale or to buy when the order was placed
    pub volume_total: i64,
}

/// Information regarding a specific market group
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/MarketsGroupsMarketGroupIdGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarketItemGroupInformation {
    /// The description of the market item group
    pub description: String,
    /// The name of the market item group
    pub name: String,
    /// The ID of the market item group
    pub market_group_id: i64,
    /// The ID of the parent market item group if applicable
    pub parent_group_id: Option<i64>,
    /// The type IDs of the items within the group
    pub types: Vec<i64>,
}

/// The average & adjusted market prices of an item
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/MarketsPricesGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarketItemPrices {
    /// The estimated price of what the item actually sells for on the market
    pub adjusted_price: Option<f64>,
    /// The average price of the item on the market
    pub average_price: Option<f64>,
    /// The type ID of the item on the market
    pub type_id: i64,
}

/// Details for a market order placed within a structure
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/MarketsStructuresStructureIdGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StructureMarketOrder {
    /// Number of days for which the order is valid
    /// starting from the issued date.
    ///
    /// An order expires at time issued + duration
    pub duration: i64,
    /// True if the order is a buy order
    #[serde(default)]
    pub is_buy_order: bool,
    /// Date and time when the order was issued
    pub issued: DateTime<Utc>,
    /// ID of the location where order was placed
    pub location_id: i64,
    /// For buy orders, the minimum quantity that will be accepted in a matching sell order
    pub min_volume: Option<i64>,
    /// Unique ID of the order
    pub order_id: i64,
    /// The cost per unit for this order
    pub price: f64,
    /// The range of the order
    pub range: MarketOrderRange,
    /// The type ID of the item in the order
    pub type_id: i64,
    /// Remaining quantity of items still for sale or buy
    pub volume_remain: i64,
    /// Quantity of items for sale or to buy when the order was placed
    pub volume_total: i64,
}

/// An entry for the market statistics of an item within a specific region on a given date
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/MarketsRegionIdHistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarketItemRegionStatistics {
    /// The average price of the item in this entry
    pub average: f64,
    /// The YYYY-MM-DD of this statistic entry
    pub date: NaiveDate,
    /// The highest price the item sold for in this entry
    pub highest: f64,
    /// The lowest price the item sold for in this enry
    pub lowest: f64,
    /// Total numbers of orders that occurred for this entry
    pub order_count: i64,
    /// The volume of the item traded for this entry
    pub volume: i64,
}
