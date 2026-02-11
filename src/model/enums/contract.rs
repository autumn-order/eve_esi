//! # EVE ESI Contract Enums
//!
//! Provides contract-related enums for EVE Online ESI

use serde::{Deserialize, Serialize};

/// Represents the type of contract
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/ContractsPublicRegionIdGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ContractType {
    /// The type of contract is unknown
    #[serde(rename = "unknown")]
    Unknown,
    /// Contract is an item exchange
    #[serde(rename = "item_exchange")]
    ItemExchange,
    /// Contract is an exchange
    #[serde(rename = "auction")]
    Auction,
    /// Contract is a courier to transfer items from one location to another
    #[serde(rename = "courier")]
    Courier,
    #[serde(rename = "loan")]
    Loan,
}
