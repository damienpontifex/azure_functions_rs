use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub(crate) struct TimerTriggerInputs {
    #[darling(default)]
    pub(crate) name: String,
    #[darling(default)]
    pub(crate) schedule: String,
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

