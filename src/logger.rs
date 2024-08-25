use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::SETTINGS;
use std::env;

pub fn init() {
    if env::var_os("RUST_LOG").is_none() {
        let app_name = env::var("CARGO_PKG_NAME").unwrap();
        let level = SETTINGS.logger.level.as_str();
        let env = format!("{app_name}={level},tower_http={level}");

        env::set_var("RUST_LOG", env);
    }

    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();
}
