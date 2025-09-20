//! # EVE ESI Market Enums
//!
//! Provides enums related to markets in EVE Online
//!
//! ## Enums
//! - [`MarketOrderRange`]: Indicates the the range of a market order
//! - [`HistoricalMarketOrderState`]: Indicates whether a historical market order expired or was cancelled
//! - [`OrderType`]: Represents the type of order when requesting a list of orders within a region

use std::fmt;

use serde::{Deserialize, Serialize};

/// Indicates the the range of a market order
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdOrdersGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MarketOrderRange {
    /// Market order has a range of within 1 jump
    #[serde(rename = "1")]
    OneJump,
    /// Market order has a range of within 2 jumps
    #[serde(rename = "2")]
    TwoJumps,
    /// Market order has a range of within 3 jumps
    #[serde(rename = "3")]
    ThreeJumps,
    /// Market order has a range of within 4 jumps
    #[serde(rename = "4")]
    FourJumps,
    /// Market order has a range of within 5 jumps
    #[serde(rename = "5")]
    FiveJumps,
    /// Market order has a range of within 10 jumps
    #[serde(rename = "10")]
    TenJumps,
    /// Market order has a range of within 20 jumps
    #[serde(rename = "20")]
    TwentyJumps,
    /// Market order has a range of within 30 jumps
    #[serde(rename = "30")]
    ThirtyJumps,
    /// Market order has a range of within 40 jumps
    #[serde(rename = "40")]
    FourtyJumps,
    /// Market order has a range of within its current region
    #[serde(rename = "region")]
    Region,
    /// Market order has a range of within its current system
    #[serde(rename = "solarsystem")]
    SolarSystem,
    /// Market order has a range of within its current station
    #[serde(rename = "station")]
    Station,
}

/// Indicates whether a historical market order expired or was cancelled
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdOrdersHistoryGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum HistoricalMarketOrderState {
    /// Market order was cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Market order has expired
    #[serde(rename = "expired")]
    Expired,
}

/// Represents the type of order when requesting a list of orders within a region
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/operations/GetMarketsRegionIdOrders>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum OrderType {
    /// Request only buy orders
    #[serde(rename = "buy")]
    Buy,
    /// Request only sell orders
    #[serde(rename = "sell")]
    Sell,
    /// Request both buy & sell orders
    #[serde(rename = "all")]
    All,
}

// Required for ESI endpoint macro URL formatting
//
// This enum is used as an argument when requesting market orders within a region
impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
            OrderType::All => "all",
        };
        write!(f, "{}", s)
    }
}
