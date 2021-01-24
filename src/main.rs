use func_types::Logger;

#[func_proc_macros::timer(name = "MyTimer", schedule = "*/5 * * * * *")]
fn my_timer_trigger(logger: &mut Logger) {
    logger.info("Hello, world".to_string());
}

fn main() {
    my_timer_trigger();
}
