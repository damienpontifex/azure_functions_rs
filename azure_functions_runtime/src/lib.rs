pub use ::actix_web;
pub use ::mime;
pub use actix_web::main as func_main;

#[macro_export]
macro_rules! func_runtime {
    ( $( $x:expr ),* ) => {
        $crate::actix_web::HttpServer::new(|| {
            $crate::actix_web::App::new()
                .app_data(
                    // Json extractor configuration for this resource.
                    $crate::actix_web::web::JsonConfig::default()
                        .content_type(|mime| mime.subtype() == $crate::mime::JSON)
                )
                $(
                .service($x)
                )*
        })
        .bind((std::net::Ipv4Addr::UNSPECIFIED, 8080)).unwrap()
        .run()
        .await
    };
}

