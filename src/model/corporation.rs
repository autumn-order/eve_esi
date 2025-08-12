use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a corporation in EVE Online.
///
/// Corporations are player-run organizations within EVE Online.
/// This struct contains information about a corporation in EVE Online, including its ID, name, ticker, and other details.
///
/// # Fields
/// The struct contains information about a corporation in EVE Online, including:
/// - `alliance_id` - The ID of the alliance which the corporation is a member of, if applicable.
/// - `ceo_id` - The ID of the corporation's CEO.
/// - `creator_id` - The ID of the character who created the corporation.
/// - `date_founded` - The date and time when the corporation was founded.
/// - `description` - The description of the corporation.
/// - `faction_id` - The ID of the faction which the corporation is a member of, if applicable.
/// - `home_station_id` - The ID of the corporation's home station.
/// - `member_count` - The number of members in the corporation.
/// - `name` - The name of the corporation.
/// - `shares` - The number of shares that exist for the corporation.
/// - `tax_rate` - The tax rate of the corporation.
/// - `ticker` - The ticker of the corporation.
/// - `url` - The URL of the corporation's website.
/// - `war_eligible` - Whether or not the corporation is eligible for war.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Corporation {
    /// The ID of the alliance which the corporation is a member of, if applicable.
    pub alliance_id: Option<i32>,
    /// The ID of the corporation's CEO.
    pub ceo_id: i32,
    /// The ID of the character who created the corporation.
    pub creator_id: i32,
    /// The date and time when the corporation was founded.
    pub date_founded: Option<DateTime<Utc>>,
    /// The description of the corporation.
    pub description: Option<String>,
    /// The ID of the faction which the corporation is a member of, if applicable.
    pub faction_id: Option<i32>,
    /// The ID of the corporation's home station.
    pub home_station_id: Option<i32>,
    /// The number of members in the corporation.
    pub member_count: i32,
    /// The name of the corporation.
    pub name: String,
    /// The number of shares that exist for the corporation.
    pub shares: Option<i64>,
    /// The tax rate of the corporation.
    pub tax_rate: f32,
    /// The ticker of the corporation.
    pub ticker: String,
    /// The URL of the corporation's website.
    pub url: Option<String>,
    /// Whether or not the corporation is eligible for war.
    pub war_eligible: Option<bool>,
}
