//! # EVE ESI Market Models
//!
//! Provides models related to market endpoints for EVE Online's ESI API.
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::enums::market::MarketOrderRange;

/// Details for a market order
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdOrdersGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarketOrder {
    /// Number of days for which the order is valid
    /// starting from the issued date.
    ///
    /// An order expires at time issued + duration
    pub duration: i64,
    /// For buy orders, the amount of ISK in escrow
    pub escrow: Option<f64>,
    /// True if the order is a buy order
    #[serde(default)]
    pub is_buy_oder: bool,
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
    /// The type ID of the item in the order
    pub type_id: i64,
    /// Remaining quantity of items still for sale or buy
    pub volume_remain: i64,
    /// Quantity of items for sale or to buy when the order was placed
    pub volume_id: i64,
}
