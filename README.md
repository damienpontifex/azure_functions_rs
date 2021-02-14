Playing around with Rust procedural macros and actix web to build some helpers on Azure Functions customer handler

_Initial development and just trialling out, but happy for contributions or feedback_

```bash
cargo build && cp target/debug/handler . && func start
```

```rust
use azure_functions_runtime::func_runtime;
use azure_functions_attributes::{timer_trigger, queue_trigger};
use azure_functions_types::{TimerInfo, QueueTrigger, Logger};
use serde::Deserialize;

#[timer_trigger(name = "MyTimer", schedule = "*/5 * * * * *")]
pub(crate) fn my_timer_trigger(timer: TimerInfo, logger: &mut Logger) {
    logger.info(format!("Timer trigger fired at {}", timer.metadata.sys.utc_now));
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueMessage {
    my_queue_item: String,
}

#[queue_trigger(name = "MyQueueTrigger", queue_name = "myqueue", connection = "AzureStorageConnectionString")]
pub(crate) fn my_queue_trigger(queue_item: QueueTrigger<QueueMessage>, logger: &mut Logger) {
    logger.info(format!("Received queue message: {:#?}", queue_item.data.my_queue_item));
}

fn main() {
    func_runtime!(timer_trigger_fn, my_queue_trigger);
}
```

host.json
```json
{
  "version": "2.0",
  "logging": {
    "applicationInsights": {
      "samplingSettings": {
        "isEnabled": true,
        "excludedTypes": "Request"
      }
    }
  },
  "extensionBundle": {
    "id": "Microsoft.Azure.Functions.ExtensionBundle",
    "version": "[1.*, 2.0.0)"
  },
  "customHandler": {
    "description": {
      "defaultExecutablePath": "handler",
      "workingDirectory": "",
      "arguments": []
    }
  }
}
```

.funcignore
```
target
```