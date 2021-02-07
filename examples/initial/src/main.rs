//mod my_timer_trigger;
//mod my_queue_trigger;
use azure_functions_runtime::func_runtime;
// use my_timer_trigger::my_timer_trigger as timer_trigger_fn;

//#[actix_web::get("/")]
//async fn my_func() -> impl actix_web::Responder {
//    "".to_string()
//}

fn main() {
    //func_runtime!(timer_trigger_fn, my_queue_trigger::run)
    func_runtime!()
}
