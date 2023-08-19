use serde::Deserialize;
use ::config::Config;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub pg: deadpool_postgres::Config
}

impl AppConfig {

    pub fn build() -> AppConfig {
        let config_ = Config::builder()
        .add_source(::config::Environment::default().separator("__"))
        .build()
        .unwrap();

        config_.try_deserialize().unwrap()
    }

}