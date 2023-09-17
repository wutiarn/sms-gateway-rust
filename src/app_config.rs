use std::collections::HashMap;
use std::ops::Deref;
use anyhow::{anyhow, Error};
use config::FileFormat;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub telegram_bot_token: String,
    pub device_to_chat_id_mapping: HashMap<String, String>,
    pub android_packages_mapping: HashMap<String, String>,
    pub log_requests: bool,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let defaults_file = std::include_str!("../app_config.defaults.yaml");
        let c = config::Config::builder()
            .add_source(config::File::from_str(defaults_file, FileFormat::Yaml))
            .add_source(config::File::with_name("app_config").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        c.try_deserialize()
    }

    pub fn get_chat_id<'t>(&'t self, device_id: &'t str) -> Result<&'t str, Error> {
        self.device_to_chat_id_mapping.get(device_id).map(|x| x.deref()).ok_or_else(|| anyhow!("Failed to find chat id for device"))
    }

    pub fn get_app_name<'t>(&'t self, package_id: &'t str) -> Option<&'t str> {
        self.android_packages_mapping.get(package_id).map(|x| x.deref())
    }
}
