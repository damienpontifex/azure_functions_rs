Playing around with Rust procedural macros and actix web to build some helpers on Azure Functions customer handler

_Initial development and just trialling out, but happy for contributions or feedback_

```rust
use azure_functions_runtime::func_runtime;
use azure_functions_attributes::{timer_trigger, queue_trigger};
use azure_functions_types::{TimerInfo, QueueTrigger, Logger};
use serde::Deserialize;

#[timer_trigger(name = "MyTimer", schedule = "*/5 * * * * *")]
pub(crate) fn my_timer_trigger(_timer: TimerInfo, logger: &mut Logger) {
    logger.info("Hello, world".to_string());
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueMessage {
    my_queue_item: String,
}

#[queue_trigger(name = "MyQueueTrigger")]
pub(crate) fn my_queue_trigger(_queue_item: QueueTrigger<QueueMessage>, logger: &mut Logger) {
    logger.info("Hello, world".to_string());
}

fn main() {
    func_runtime!(timer_trigger_fn, my_queue_trigger)
}
```
