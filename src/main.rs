use actix_web::{middleware, web, App, HttpServer};

mod api;
mod domain;
mod model;
mod protocol;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(web::Data::new(model::Model::build()))
            .service(api::get_item)
            .service(api::get_items)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
