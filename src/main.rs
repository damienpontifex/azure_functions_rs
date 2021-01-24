use actix_web::main as func_main;
mod my_timer_trigger;
use my_timer_trigger::my_timer_trigger;

macro_rules! func_runtime {
    ( $( $x:expr ),* ) => {
        actix_web::HttpServer::new(|| {
            actix_web::App::new()
                $(
                .service($x())
                )*
        })
        .bind((std::net::Ipv4Addr::UNSPECIFIED, 8080)).unwrap()
        .run()
        .await
    };
}

#[func_main]
async fn main() -> std::io::Result<()> {
    func_runtime!(my_timer_trigger, my_timer_trigger)
}

