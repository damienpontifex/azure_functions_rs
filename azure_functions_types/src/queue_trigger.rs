use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};

pub fn double_serialized<'de, V, D>(deserializer: D) -> Result<V, D::Error>
where
    V: DeserializeOwned,
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    let first_decode: serde_json::Value = serde_json::from_str(&buf).map_err(de::Error::custom)?;
    let string_decode = first_decode
        .as_str()
        .ok_or(de::Error::custom("Initial deserialize wasn't a string"))?;
    serde_json::from_str(string_decode).map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "")]
pub struct QueueTrigger<T> where T: DeserializeOwned {
    pub data: Data<T>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data<T> where T: DeserializeOwned {
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


