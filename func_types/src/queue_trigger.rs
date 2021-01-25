use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueueTrigger<T> {
    pub data: T,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    pub dequeue_count: String,
    pub expiration_time: DateTime<Utc>,
    pub id: String,
    pub insertion_time: DateTime<Utc>,
    pub next_visible_time: DateTime<Utc>,
    pub pop_receipt: String,
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


