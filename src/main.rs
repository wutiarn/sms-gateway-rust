#[macro_use]
extern crate rocket;

use env_logger::Target;
use log::{info, LevelFilter};
use rocket::http::Status;
use rocket::State;

use dto::SmsMessageDto;
use telegram::TelegramClient;

use crate::app_config::AppConfig;
use crate::error::AppError;

mod dto;
mod telegram;
mod app_config;
mod error;

#[post("/hooks/sms", data = "<message>")]
async fn handle_sms(
    message: SmsMessageDto,
    tg: &State<TelegramClient>,
    app_config: &State<AppConfig>,
) -> Result<(Status, &'static str), AppError> {
    info!("Hook payload: {}", serde_json::to_string(&message)
        .unwrap_or_else(|e| { e.to_string() }));
    if !message.validate_secret(&app_config.api_secret) {
        return Ok((Status::Forbidden, "Token is incorrect"));
    }
    let tg_message_text = format!("{}\n---\n{}", message.body, message.from);
    tg.send_notification(&tg_message_text).await?;
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

    rocket::build()
        .mount("/", routes![handle_sms])
        .manage(telegram_client)
        .manage(app_config)
}
