use azure_functions_attributes::queue_trigger;
use azure_functions_types::{QueueTrigger, Logger};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct QueueItem {
    message: String,
}

#[queue_trigger(name = "MyQueueTrigger", queue_name = "myqueue", connection = "AzureStorageConnectionString")]
pub(crate) fn run(message: QueueTrigger<QueueItem>, logger: &mut Logger) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    logger.info(format!("Received queue message: {:#?}", message.data.queue_item));
    Ok(())
}

