use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use func_types::Logger;

#[func_proc_macros::timer(name = "MyTimer", schedule = "*/5 * * * * *")]
fn my_timer_trigger(logger: &mut func_types::Logger) {
    logger.info("Hello, world".to_string());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(my_timer_trigger())
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
    })
    .bind((std::net::Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}

//fn main() {
//    my_timer_trigger();
//}
