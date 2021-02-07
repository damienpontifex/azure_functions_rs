use azure_functions_attributes::timer_trigger;
use func_types::Logger;

#[timer_trigger(name = "MyTimer", schedule = "*/5 * * * * *")]
pub(crate) fn my_timer_trigger(_timer: func_types::TimerInfo, logger: &mut Logger) {
    logger.info("Hello, world".to_string());
}
