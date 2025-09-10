//! Data structures and types for representing corporations in EVE Online.
//!
//! This module defines the `Corporation` struct, which models the core properties of a corporation in EVE Online.
//!
//! See [ESI API documentation](https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdGet)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a corporation in EVE Online.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CorporationsCorporationIdGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Corporation {
    /// The ID of the alliance which the corporation is a member of, if applicable.
    pub alliance_id: Option<i64>,
    /// The ID of the corporation's CEO.
    pub ceo_id: i64,
    /// The ID of the character who created the corporation.
    pub creator_id: i64,
    /// The date and time when the corporation was founded.
    pub date_founded: Option<DateTime<Utc>>,
    /// The description of the corporation.
    pub description: Option<String>,
    /// The ID of the faction which the corporation is a member of, if applicable.
    pub faction_id: Option<i64>,
    /// The ID of the corporation's home station.
    pub home_station_id: Option<i64>,
    /// The number of members in the corporation.
    pub member_count: i64,
    /// The name of the corporation.
    pub name: String,
    /// The number of shares that exist for the corporation.
    pub shares: Option<i64>,
    /// The tax rate of the corporation.
    pub tax_rate: f64,
    /// The ticker of the corporation.
    pub ticker: String,
    /// The URL of the corporation's website.
    pub url: Option<String>,
    /// Whether or not the corporation is eligible for war.
    pub war_eligible: Option<bool>,
}
