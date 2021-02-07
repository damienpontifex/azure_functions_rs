mod my_timer_trigger;
mod my_queue_trigger;
use func_runtime::{func_main, func_runtime};
use my_timer_trigger::my_timer_trigger as timer_trigger_fn;

//#[actix_web::get("/")]
//async fn my_func() -> impl actix_web::Responder {
//    "".to_string()
//}

#[func_main]
async fn main() -> std::io::Result<()> {
    func_runtime!(timer_trigger_fn, my_queue_trigger::run)
}
