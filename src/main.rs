#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::error::Error;

use rocket::http::Status;
use rocket::response::status::Accepted;
use rocket::response::status::Custom;
use rocket::State;

use dto::SmsMessageDto;
use telegram::TelegramClient;
use crate::app_config::{AppConfig};

mod dto;
mod telegram;
mod app_config;

#[post("/hooks/sms", data = "<message>")]
fn index(
    message: SmsMessageDto,
    tg: &State<TelegramClient>,
    app_config: &State<AppConfig>
) -> Result<&'static str, Custom<&'static str>> {
    println!("Body is: {:?}", message);
    if !message.validate_secret(&app_config.api_secret) {
        return Err(Custom(Status::Forbidden, "Token is incorrect"));
    }
    tg.send_notification("test");
    Ok("OK")
}

#[launch]
fn rocket() -> _ {
    let app_config = match AppConfig::new() {
        Ok(it) => it,
        Err(e) => panic!("Failed to construct app config: {}", e)
    };
    println!("{:?}", app_config);

    let telegram_client = TelegramClient {
        bot_api_token: app_config.telegram_bot_token.clone()
    };

    rocket::build()
        .mount("/", routes![index])
        .manage(telegram_client)
        .manage(app_config)
}
