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
    buyout: Option<f64>,
    /// Collateral (for Couriers only)
    collateral: Option<f64>,
    /// The EVE Online ID of the contract
    contract_id: i64,
    /// The expiration date of the contract
    date_expired: DateTime<Utc>,
    /// Creation date of the contract
    date_issued: DateTime<Utc>,
    /// Number of days to complete the contract
    days_to_complete: Option<i64>,
    /// End location ID (for Couriers only)
    end_location_id: Option<i64>,
    /// true if the corporation was issueed on behalf of the issuer's corporation
    #[serde(default)]
    for_corporation: bool,
    /// Character's corporation ID for the issuer
    issuer_corporation_id: i64,
    /// Character ID for the issuer
    issuer_id: i64,
    /// The price of the contract (for ItemExchange & Auction)
    price: Option<f64>,
    /// The reward for completing the contract (for Couriers only)
    reward: Option<f64>,
    /// The location ID of the contract
    /// - ESI documents this improperly, the start location is actually shown for
    /// item exchanges as well as couriers.
    start_location_id: Option<i64>,
    /// Title of the contract
    title: Option<String>,
    /// Type of the contract
    r#type: ContractType,
    /// Volume of items in the contract
    volume: Option<u64>,
}
