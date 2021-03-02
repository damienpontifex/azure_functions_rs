use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimerInfo {
    pub data: Data,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub timer: Timer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timer {
    pub schedule: Schedule,
    pub schedule_status: Option<serde_json::Value>,
    pub is_past_due: bool,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    #[serde(rename = "AdjustForDST")]
    pub adjust_for_dst: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    #[serde(rename = "sys")]
    pub sys: Sys,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sys {
    pub method_name: String,
    pub utc_now: DateTime<Utc>,
    pub rand_guid: Uuid,
}
