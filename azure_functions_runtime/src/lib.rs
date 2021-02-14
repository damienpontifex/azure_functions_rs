pub use ::mime;
pub use ::actix_web;

#[macro_export]
macro_rules! func_runtime {
    ( $( $x:expr ),* ) => {
        $crate::actix_web::rt::System::new("main").block_on(async move {
            let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
            let port: u16 = match std::env::var(port_key) {
                Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
                Err(_) => 3000,
            };

            $crate::actix_web::HttpServer::new(|| {
                $crate::actix_web::App::new()
                    .app_data(
                        // Json extractor configuration for this resource.
                        $crate::actix_web::web::JsonConfig::default()
                            .content_type(|mime| mime.subtype() == $crate::mime::JSON)
                    )
                    .route("/", $crate::actix_web::web::get().to(|| $crate::actix_web::HttpResponse::Ok()))
                    $(
                    .service($x)
                    )*
            })
            .bind((std::net::Ipv4Addr::UNSPECIFIED, port)).unwrap()
            .run()
            .await
        }).unwrap();
    };
}
