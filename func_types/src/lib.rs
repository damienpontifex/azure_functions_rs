mod timer_info;
pub use timer_info::TimerInfo;

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
