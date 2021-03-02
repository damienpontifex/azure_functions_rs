use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};

pub(crate) fn double_serialized<'de, V, D>(deserializer: D) -> Result<V, D::Error>
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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    struct QueueItem {
        message: String,
    }
    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    struct Data {
        #[serde(deserialize_with = "double_serialized")]
        my_queue_item: QueueItem,
    }

    #[test]
    fn embedded_queue_item_is_properly_deserialized() {
        let data_message = r#"{"myQueueItem":"\"{ \\\"message\\\": \\\"my message\\\" }\""}"#;
        let data: Result<Data, _> = serde_json::from_str(data_message);

        assert_eq!(data.is_ok(), true);
        assert_eq!(
            data.unwrap(),
            Data {
                my_queue_item: QueueItem {
                    message: "my message".to_string(),
                },
            }
        );
    }
}
