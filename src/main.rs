#[macro_use]
extern crate rocket;

use std::net::IpAddr;
use std::str::FromStr;
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

#[post("/api/hook/sms", data = "<message>")]
async fn handle_sms(
    message: Json<SmsMessagesDto>,
    tg: &State<TelegramClient>,
    app_config: &State<AppConfig>,
) -> Result<(Status, &'static str), HttpError> {
    let dto = message;
    // info!("Hook payload: {}", serde_json::to_string(&dto)
    //     .unwrap_or_else(|e| { e.to_string() }));
    // if !message.validate_secret(&app_config.api_secret) {
    //     return HttpError::new(anyhow!("Token is incorrect"))
    //         .with_status(Status::Forbidden)
    //         .err();
    // }
    // let tg_message_text = format!("{}\n---\n{}", message.body, message.from);
    // tg.send_notification(&tg_message_text).await?;
    Ok((Status::Ok, "OK"))
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
