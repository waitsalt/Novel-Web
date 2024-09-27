use std::env;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::setting::SETTING;

pub fn init() {
    if env::var_os("RUST_LOG").is_none() {
        let app_name = env::var("CARGO_PKG_NAME").unwrap();
        let level = SETTING.logger.level.as_str();
        let env = format!("{app_name}={level},tower_http={level}");

        env::set_var("RUST_LOG", env);
    }

    tracing_subscriber::registry().with(fmt::layer()).init();
}
