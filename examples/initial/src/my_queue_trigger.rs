use azure_functions_attributes::queue_trigger;
use azure_functions_types::{QueueTrigger, Logger};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueMessage {
    my_queue_item: String,
}

#[queue_trigger(name = "MyQueueTrigger")]
pub(crate) fn run(_timer: QueueTrigger<QueueMessage>, logger: &mut Logger) {
    logger.info("Hello, world".to_string());
}
