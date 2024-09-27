use once_cell::sync::Lazy;
use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub server: Server,
    pub database: Database,
    pub auth: Auth,
    pub logger: Logger,
}

pub static SETTING: Lazy<Setting> = Lazy::new(|| Setting::init().expect("Fail to setup settig"));

impl Setting {
    pub fn init() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::default().separator("_"));

        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        builder.build()?.try_deserialize()
    }
}
