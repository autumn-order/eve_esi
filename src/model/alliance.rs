use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Alliance {
    pub creator_corporation_id: i32,
    pub creator_id: i32,
    pub date_founded: DateTime<Utc>,
    pub executor_corporation_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub name: String,
    pub ticker: String,
}
