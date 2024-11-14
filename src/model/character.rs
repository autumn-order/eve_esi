use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Character {
    pub alliance_id: Option<i32>,
    pub birthday: DateTime<Utc>,
    pub bloodline_id: i32,
    pub corporation_id: i32,
    pub description: Option<String>,
    pub faction_id: Option<i32>,
    pub gender: String,
    pub name: String,
    pub race_id: i32,
    pub security_status: Option<f32>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CharacterAffiliation {
    pub alliance_id: Option<i32>,
    pub character_id: i32,
    pub corporation_id: i32,
    pub faction_id: Option<i32>,
}
