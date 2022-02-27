#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(actix_web::web::resource("/").to(|| async {
                actix_web::HttpResponse::build(actix_web::http::StatusCode::IM_A_TEAPOT)
                    .insert_header(("server", "timpaik"))
                    .content_type("text/plain")
                    .body("418. I’m a teapot.")
            }))
    })
    .bind(("0.0.0.0", 35470))?
    .run()
    .await
}
