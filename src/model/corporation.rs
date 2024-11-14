use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Corporation {
    pub alliance_id: Option<i32>,
    pub ceo_id: i32,
    pub creator_id: i32,
    pub date_founded: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub faction_id: Option<i32>,
    pub home_station_id: Option<i32>,
    pub member_count: i32,
    pub name: String,
    pub shares: Option<i64>,
    pub tax_rate: f32,
    pub ticker: String,
    pub url: Option<String>,
    pub war_eligible: Option<bool>,
}
