use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_secret: String,
    pub telegram_bot_token: String,
    pub telegram_recipient_id: String,
    pub device_to_chat_id_mapping: HashMap<String, String>
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let c = config::Config::builder()
            .add_source(config::File::with_name("app_config").required(false))
            .add_source(config::File::with_name("app_config_local").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        c.try_deserialize()
    }
}
