use darling::FromMeta;

pub(crate) trait Binding {
    fn function_name(&self) -> String;
    fn generate_json(&self) -> serde_json::Value;
}

#[derive(Debug, FromMeta)]
pub(crate) struct TimerTriggerInputs {
    #[darling(default)]
    pub(crate) name: String,
    #[darling(default)]
    pub(crate) schedule: String,
}

impl Binding for TimerTriggerInputs {
    fn function_name(&self) -> String {
        self.name.clone()
    }

    fn generate_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "timerTrigger",
            "direction": "in",
            "name": "timer",
            "schedule": self.schedule,
        })
    }
}


#[derive(Debug, FromMeta)]
pub(crate) struct EventGridTriggerInputs {
    #[darling(default)]
    name: String,
}

#[derive(Debug, FromMeta)]
pub(crate) struct QueueTriggerInputs {
    #[darling(default)]
    pub(crate) name: String,
    #[darling(default)]
    pub(crate) queue_name: String,
    #[darling(default)]
    pub(crate) connection: String,
}

impl Binding for QueueTriggerInputs {
    fn function_name(&self) -> String {
        self.name.clone()
    }

    fn generate_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "queueTrigger",
            "direction": "in",
            "name": "myQueueItem",
            "queueName": self.queue_name,
            "connection": self.connection,
        })
    }
}

#[derive(Debug, FromMeta)]
pub(crate) struct EventHubTriggerInputs {
    #[darling(default)]
    name: String,
    #[darling(default)]
    event_hub_name: String,
    #[darling(default)]
    connection: String,
}

#[derive(Debug, FromMeta)]
pub(crate) struct BlobStorageTriggerInputs {
    #[darling(default)]
    name: String,
    #[darling(default)]
    path: String,
    #[darling(default)]
    connection: String,
}

