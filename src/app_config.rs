use config::{Config, ConfigError, Source, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_secret: String,
    pub telegram_bot_token: String
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::default();
        config
            .merge(config::File::with_name("app_config"))?
            .merge(config::Environment::with_prefix("APP"))?;
        config.try_into()
    }
}
