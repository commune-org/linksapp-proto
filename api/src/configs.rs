use crate::errors::ServiceError;
use anyhow::{format_err, Error};
pub use config::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

pub const KEY_LENGTH: usize = 32;

type SecretKey = [u8; KEY_LENGTH];

fn default_expiration_seconds() -> i64 {
    24 * 3600
}

#[derive(Deserialize)]
pub struct Cookies {
    pub secret_key: u8,
    pub expiration_seconds: i64,
}

#[derive(Deserialize)]
pub struct DomainConfig {
    pub url: String,
}

// TODO change to correct gogole client id
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecureConfig {
    #[serde(default = "default_secret_path")]
    pub secret_path: PathBuf,
    #[serde(default = "default_secret_path")]
    pub jwt_secret_path: PathBuf,
    #[serde(default = "default_key")]
    pub google_client_id: String,
    #[serde(default = "default_key")]
    pub google_client_secret: String,
}

fn default_secret_path() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        // .join("aws_app_rust")
        .join("secret.bin")
}

fn default_key() -> String {
    "0123".repeat(8).into()
}

#[derive(Deserialize, Clone)]
pub struct MailConfig {
    pub driver: String,
    pub host: String,
    pub port: String,
    pub username: String,
    pub sender: String,
    pub encryption: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub registration_callback: String,
    pub port: i32,
    pub secret_key: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub srv_cnf: ServerConfig,
    pub mail_cnf: MailConfig,
    // pub ggle_clnt: SecureConfig,
    // pub domain_cnf: DomainConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build();
    }
}

#[derive(Debug, Clone)]
pub struct SecConfig(Arc<SecureConfig>);

impl Deref for SecConfig {
    type Target = SecureConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SecConfig {
    pub fn from_inner(inner: SecureConfig) -> Self {
        Self(Arc::new(inner))
    }

    pub fn init_config() -> Result<Self, ServiceError> {
        let fname = Path::new("config.env");
        let config_dir = dirs::config_dir().ok_or_else(|| format_err!("No CONFIG directory"))?;
        let default_fname = config_dir.join("config.env");

        let env_file = if fname.exists() {
            fname
        } else {
            &default_fname
        };

        dotenv::dotenv().ok();

        if env_file.exists() {
            dotenv::from_path(env_file).ok();
        }

        let conf: SecureConfig = envy::from_env()?;

        Ok(Self::from_inner(conf))
    }
}
