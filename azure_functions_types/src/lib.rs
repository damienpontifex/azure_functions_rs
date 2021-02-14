mod timer_info;
pub use timer_info::TimerInfo;

mod queue_trigger;
pub use queue_trigger::QueueTrigger;

use serde::Serialize;

//
#[derive(Default)]
pub struct Logger {
    //
    pub messages: Vec<String>,
}

impl Logger {
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
