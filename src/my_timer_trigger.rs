use func_types::Logger;

#[func_proc_macros::timer(name = "MyTimer", schedule = "*/5 * * * * *")]
pub(crate) fn my_timer_trigger(logger: &mut Logger) {
    logger.info("Hello, world".to_string());
}

