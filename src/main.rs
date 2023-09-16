#[macro_use]
extern crate rocket;

use std::net::IpAddr;
use std::str::FromStr;
use anyhow::{anyhow, Error};
use env_logger::Target;
use log::{info, LevelFilter};
use rocket::State;
use rocket::figment::Figment;
use rocket::figment::providers::Env;
use rocket::http::Status;
use rocket::serde::json::Json;

use dto::SmsMessagesDto;
use telegram::TelegramClient;

use crate::app_config::AppConfig;
use crate::error::HttpError;

mod dto;
mod telegram;
mod app_config;
mod error;

#[post("/api/hook/sms", data = "<dto>")]
async fn handle_sms<'t>(
    dto: Json<SmsMessagesDto>,
    tg: &State<TelegramClient>,
    app_config: &State<AppConfig>,
) -> Result<(Status, &'static str), HttpError> {
    let dto = dto.into_inner();
    info!("Hook payload: {}", serde_json::to_string(&dto).unwrap());
    let chat_id = get_chat_id(app_config, &dto.device_id)?;
    for msg in dto.messages {
        let tg_message_text = format!("{}\n---\n{} ({})", msg.message, msg.from, dto.carrier_name);
        tg.send_notification(chat_id, &tg_message_text).await?;
    }
    Ok((Status::Ok, "OK"))
}

fn get_chat_id<'t>(app_config: &'t AppConfig, device_id: &'t str) -> Result<&'t str, Error> {
    if let Some(found) = app_config.device_to_chat_id_mapping.get(device_id) {
        return Ok(found);
    }
    return Err(anyhow!("Failed to find chat id for device"))
}

#[launch]
fn rocket() -> _ {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let app_config = match AppConfig::new() {
        Ok(it) => it,
        Err(e) => panic!("Failed to construct app config: {}", e)
    };
    info!("{:#?}", app_config);

    let telegram_client = TelegramClient::new(
        app_config.telegram_bot_token.clone(),
        app_config.telegram_recipient_id.clone(),
    );

    let mut config = rocket::Config::default();
    config.port = 8080;
    config.address = IpAddr::from_str("0.0.0.0").unwrap();

    let figment = Figment::from(config)
        .merge(Env::prefixed("ROCKET_").global());

    rocket::custom(figment)
        .mount("/", routes![handle_sms])
        .manage(telegram_client)
        .manage(app_config)
}
