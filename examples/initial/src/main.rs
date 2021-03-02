mod my_timer_trigger;
mod my_queue_trigger;
use azure_functions_runtime::func_runtime;
use my_timer_trigger::my_timer_trigger as timer_trigger_fn;

fn main() {
    func_runtime!(my_queue_trigger::run, timer_trigger_fn);
}
