use azure_functions_attributes::timer_trigger;
use azure_functions_types::{TimerInfo, Logger};

#[timer_trigger(name = "MyTimer", schedule = "*/5 * * * * *")]
pub(crate) fn my_timer_trigger(timer: TimerInfo, logger: &mut Logger) {
    logger.info(format!("Timer trigger fired at {}", timer.metadata.sys.utc_now));
}
