use crate::serialization::double_serialized;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};
use uuid::Uuid;

/// A Message object which is stored in a Queue
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "")]
pub struct QueueTrigger<T>
where
    T: DeserializeOwned,
{
    /// The content of the trigger
    pub data: Data<T>,
    /// Metadata about the Message from the queue
    pub metadata: Metadata,
}

/// Content of the Queue Message
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data<T>
where
    T: DeserializeOwned,
{
    /// The content of the message
    #[serde(deserialize_with = "double_serialized")]
    pub queue_item: T,
}

/// Azure storage queue metadata
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    /// The number of times the message has been dequeued
    pub dequeue_count: String,
    /// The time that the Message will expire and be automatically deleted
    pub expiration_time: DateTime<Utc>,
    /// The Id of the Message
    pub id: String,
    /// The time the Message was inserted into the Queue
    pub insertion_time: DateTime<Utc>,
    /// The time that the message will again become visible in the Queue
    pub next_visible_time: DateTime<Utc>,
    /// This value is required to delete the Message. If deletion fails using this popreceipt then the message has been dequeued by another client.
    pub pop_receipt: String,
    /// Function invocation system properties
    #[serde(rename = "sys")]
    pub sys: Sys,
}

/// System information about Azure Function invocation
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sys {
    /// Method that was invoked with the Queue Message
    pub method_name: String,
    /// UTC now of when the function was invoked
    pub utc_now: DateTime<Utc>,
    /// Unique guid for invocation
    pub rand_guid: Uuid,
}
