//! # EVE ESI Contract Models
//!
//! Provides contract-related structs for EVE Online

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::enums::contract::ContractType;

/// A public contract's information
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/ContractsPublicRegionIdGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PublicContract {
    /// Buyout price (for Auctions only)
    pub buyout: Option<f64>,
    /// Collateral (for Couriers only)
    pub collateral: Option<f64>,
    /// The EVE Online ID of the contract
    pub contract_id: i64,
    /// The expiration date of the contract
    pub date_expired: DateTime<Utc>,
    /// Creation date of the contract
    pub date_issued: DateTime<Utc>,
    /// Number of days to complete the contract
    pub days_to_complete: Option<i64>,
    /// End location ID (for Couriers only)
    pub end_location_id: Option<i64>,
    /// true if the corporation was issueed on behalf of the issuer's corporation
    #[serde(default)]
    pub for_corporation: bool,
    /// Character's corporation ID for the issuer
    pub issuer_corporation_id: i64,
    /// Character ID for the issuer
    pub issuer_id: i64,
    /// The price of the contract (for ItemExchange & Auction)
    pub price: Option<f64>,
    /// The reward for completing the contract (for Couriers only)
    pub reward: Option<f64>,
    /// The location ID of the contract
    /// - ESI documents this improperly, the start location is actually shown for
    /// item exchanges as well as couriers.
    pub start_location_id: Option<i64>,
    /// Title of the contract
    pub title: Option<String>,
    /// Type of the contract
    pub r#type: ContractType,
    /// Volume of items in the contract
    pub volume: Option<u64>,
}

/// Represents an item entry for a public contract
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/ContractsPublicItemsContractIdGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PublicContractItem {
    /// True is item is a blueprint copy
    #[serde(default)]
    pub is_blueprint_copy: bool,
    /// True if the contract issuer has submitted this item with the contract,
    /// false if the issuer is asking for this item in the contract
    pub is_included: bool,
    /// Unique ID of the item being sold. None if the item is
    /// being requested by contract rather than sold with contract.
    pub item_id: Option<i64>,
    /// Material efficiency level of the blueprint, None if not a blueprint
    pub material_efficiency: Option<i64>,
    /// The quantity of items in the stack
    pub quantity: i64,
    /// Unique ID for the item, used by the contract system
    pub record_id: i64,
    /// Number of runs if the blueprint is a copy
    /// - -1 if it is an original
    /// - None if it is not a blueprint
    pub runs: Option<i64>,
    /// Time efficiency level of the blueprint, None if not a blueprint
    pub time_efficiency: Option<i64>,
    /// Type ID for the item
    pub type_id: i64,
}
