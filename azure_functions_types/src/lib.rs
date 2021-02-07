mod timer_info;
pub use timer_info::TimerInfo;

mod queue_trigger;
pub use queue_trigger::QueueTrigger;

use serde::Serialize;

#[derive(Default)]
pub struct Logger {
    messages: Vec<String>,
}

impl Logger {
    pub fn info(&mut self, msg: String) {
        self.messages.push(msg);
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        println!("Log messages");
        for msg in &self.messages {
            println!("{}", msg);
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuncResponse {
    // pub outputs: serde_json::Value,
    pub logs: Vec<String>,
    // pub return_value: String,
}
