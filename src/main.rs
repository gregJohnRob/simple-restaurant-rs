use actix_web::{middleware, web, App, HttpServer};
use tokio_postgres::NoTls;

mod api;
mod domain;
mod model;
mod protocol;

mod config;

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = config::AppConfig::build();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let model = model::Model::build(pool);

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(web::Data::new(model.clone()))
            .service(api::get_item)
            .service(api::get_items)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
