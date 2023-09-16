use std::collections::HashMap;
use std::ops::Deref;
use anyhow::{anyhow, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub telegram_bot_token: String,
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

    pub fn get_chat_id<'t>(&'t self, device_id: &'t str) -> Result<&'t str, Error> {
        self.device_to_chat_id_mapping.get(device_id).map(|x| x.deref()).ok_or_else(|| anyhow!("Failed to find chat id for device"))
    }
}
