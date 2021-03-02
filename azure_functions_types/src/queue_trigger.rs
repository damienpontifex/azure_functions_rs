use crate::serialization::double_serialized;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "")]
pub struct QueueTrigger<T>
where
    T: DeserializeOwned,
{
    pub data: Data<T>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data<T>
where
    T: DeserializeOwned,
{
    #[serde(deserialize_with = "double_serialized")]
    pub queue_item: T,
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
