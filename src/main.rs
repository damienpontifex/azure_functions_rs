mod my_timer_trigger;
use my_timer_trigger::my_timer_trigger as timer_trigger_fn;
use func_runtime::{func_main, func_runtime};

//#[actix_web::get("/")]
//async fn my_func() -> impl actix_web::Responder {
//    "".to_string()
//}

#[func_main]
async fn main() -> std::io::Result<()> {
    func_runtime!(timer_trigger_fn)
}

