use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SrvConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub srv_cnf: SrvConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
    }
}
