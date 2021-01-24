
#[func_proc_macros::timer(name = "MyTimer", schedule = "*/5 * * * * *")]
fn my_timer_trigger() {
    println!("Hello, world");
}

fn main() {
    my_timer_trigger();
}
