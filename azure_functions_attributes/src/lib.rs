#![feature(box_patterns)]
use proc_macro::TokenStream;
mod inputs;
use inputs::{QueueTriggerInputs, TimerTriggerInputs};

mod trigger;

/// A timer trigger lets you run a function on a schedule
///
/// # Syntax
/// ```rust
/// #[timer_trigger(name = "UniqueFunctionName", schedule = "ScheduleExpression")]
/// ```
///
/// # Attributes
/// - `name` - A unique function name that will show up in the Azure portal
/// - `schedule` - CRON expression. You can put the schedule expression in an app setting and set this property to the app setting name wrapped in % signs, as in this example: "%ScheduleAppSetting%".
///
/// # Example
///
/// ```rust
/// #[timer_trigger(name = "MyTimer", schedule = "*/5 * * * * *")]
/// fn my_timer_trigger(timer: TimerInfo, logger: &mut Logger) {
///     logger.info(format!("Timer trigger fired at {}", timer.metadata.sys.utc_now));
/// }
/// ```
///
/// Reference: <https://docs.microsoft.com/en-us/azure/azure-functions/functions-bindings-timer>
#[proc_macro_attribute]
pub fn timer_trigger(args: TokenStream, item: TokenStream) -> TokenStream {
    trigger::impl_trigger::<TimerTriggerInputs>(args, item, "TimerInfo").into()
}

// #[proc_macro_attribute]
// pub fn event_grid_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }

// #[proc_macro_attribute]
// pub fn blob_storage_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }

// #[proc_macro_attribute]
// pub fn notification_hub_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }

/// Run a function as queue storage data changes
///
/// # Syntax
/// ```rust
/// #[queue_trigger(name = "MyQueueTrigger", queue_name = "funcqueue", connection = "AzureWebJobsStorage")]
/// ```
///
/// # Attributes
/// - `name` - A unique function name that will show up in the Azure portal
/// - `queue_name` - The name of the queue to poll
/// - `connection` - The name of an app setting that contains the Storage connection string to use for this binding. If the app setting name begins with "AzureWebJobs", you can specify only the remainder of the name here.
///
/// # Example
///
/// ```rust
/// #[derive(Deserialize, Debug)]
/// struct QueueItem {
///     message: String,
/// }
///
/// #[queue_trigger(name = "MyQueueTrigger", queue_name = "funcqueue", connection = "AzureWebJobsStorage")]
/// fn run(message: QueueTrigger<QueueItem>, logger: &mut Logger) {
///     logger.info(format!("Received queue message: {:#?}", message));
/// }
/// ```
///
/// Reference: <https://docs.microsoft.com/en-us/azure/azure-functions/functions-bindings-storage-queue-trigger>
#[proc_macro_attribute]
pub fn queue_trigger(args: TokenStream, item: TokenStream) -> TokenStream {
    trigger::impl_trigger::<QueueTriggerInputs>(args, item, "QueueTrigger").into()
}
