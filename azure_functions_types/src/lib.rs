mod timer_info;
pub use timer_info::TimerInfo;

mod queue_trigger;
pub use queue_trigger::QueueTrigger;

mod serialization;

use serde::Serialize;

/// Azure function logger
#[derive(Default)]
pub struct Logger {
    /// Logged messages for each function invocation
    pub messages: Vec<String>,
}

impl Logger {
    /// Log message with information level
    pub fn info(&mut self, msg: String) {
        self.messages.push(msg);
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuncResponse {
    pub outputs: serde_json::Value,
    pub logs: Vec<String>,
    pub return_value: String,
}

impl FuncResponse {
    pub fn new() -> Self {
        FuncResponse {
            outputs: serde_json::json!({ "res": { "body": "" }}),
            ..Default::default()
        }
    }
}
