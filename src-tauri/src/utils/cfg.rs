use config::{self, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub addr: String,
    pub client_addr: String,
    pub boss_secret: String,
    pub api_secret: String,
    pub page_size: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cfg {
    pub server: Server,
    pub database_url: String,
    pub redis_url: String,
}

impl Cfg {
    pub fn from_env() -> Result<Self, ConfigError> {
        ::config::Config::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize()
    }

    pub fn get(&self) -> &Self {
        &self
    }
}
