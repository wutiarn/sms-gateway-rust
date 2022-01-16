use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_secret: String,
    pub telegram_bot_token: String,
    pub telegram_recipient_id: String
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut config = config::Config::default();
        config.merge(config::File::with_name("app_config").required(false))?;
        config.merge(config::File::with_name("app_config_local").required(false))?;

        config.merge(config::Environment::with_prefix("APP"))?;
        config.try_into()
    }
}
