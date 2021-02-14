use azure_functions_attributes::queue_trigger;
use azure_functions_types::{QueueTrigger, Logger};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueMessage {
    my_queue_item: String,
}

#[queue_trigger(name = "MyQueueTrigger", queue_name = "myqueue", connection = "AzureStorageConnectionString")]
pub(crate) fn run(message: QueueTrigger<QueueMessage>, logger: &mut Logger) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    logger.info(format!("Received queue message: {:#?}", message.data.my_queue_item));
    Ok(())
}

