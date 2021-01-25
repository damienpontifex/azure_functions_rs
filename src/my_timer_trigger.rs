use func_proc_macros::timer_trigger;
use func_types::{Logger, TimerInfo};

#[timer_trigger(name = "MyTimer", schedule = "*/5 * * * * *")]
pub(crate) fn my_timer_trigger(_timer: TimerInfo, logger: &mut Logger) {
    logger.info("Hello, world".to_string());
}

