/*
Copyright 2022 Tim-Paik <timpaik@163.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
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
                    .body("418. I'm a teapot.")
            }))
    })
    .bind(("0.0.0.0", 35470))?
    .run()
    .await
}
