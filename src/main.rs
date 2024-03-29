#[macro_use]
extern crate rocket;

use std::net::IpAddr;
use std::str::FromStr;

use env_logger::Target;
use log::{info, LevelFilter};
use rocket::figment::Figment;
use rocket::figment::providers::Env;
use rocket::http::Status;
use rocket::State;

use dto::sms_message::SmsMessagesDto;
use telegram::TelegramClient;

use crate::app_config::AppConfig;
use crate::dto::notification::NotificationDto;
use crate::error::HttpError;
use crate::classifier::MessageCategory;

mod dto;
mod telegram;
mod app_config;
mod error;
mod classifier;

#[post("/api/hook/sms", data = "<request>")]
async fn handle_sms<'t>(
    request: &str,
    tg: &State<TelegramClient>,
    app_config: &State<AppConfig>,
) -> Result<(Status, &'static str), HttpError> {
    if app_config.log_requests {
        info!("SMS report: {}", request);
    }
    let dto: SmsMessagesDto = serde_json::from_str(request)?;
    let chat_id = app_config.get_chat_id(&dto.device_id)?;
    for msg in dto.messages {
        let category = classifier::get_sms_category(&msg);
        if let MessageCategory::Ignored = category {
            info!("Ignoring message: {}", &request);
            continue;
        }
        let tg_message_text = format!("{}\n---\n{} ({})", msg.message, msg.from, dto.carrier_name);
        tg.send_notification(chat_id, &tg_message_text).await?;
    }
    Ok((Status::Ok, "OK"))
}

#[post("/api/hook/notification", data = "<request>")]
async fn handle_notification<'t>(
    request: &str,
    tg: &State<TelegramClient>,
    app_config: &State<AppConfig>,
) -> Result<(Status, &'static str), HttpError> {
    if app_config.log_requests {
        info!("Notification report: {}", request);
    }
    let dto: NotificationDto = serde_json::from_str(request)?;
    let chat_id = app_config.get_chat_id(&dto.device_id)?;

    let app_name = app_config.get_app_name(&dto.package_name);
    if app_name.is_none() {
        return Ok((Status::Ok, "Package is ignored"));
    }
    let app_name = app_name.unwrap();

    if let None = dto.text {
        info!("Missing notification text: {:?}", &dto);
        return Ok((Status::Ok, "Missing notification text"));
    }

    let category = classifier::get_notification_category(&dto);
    if let MessageCategory::Ignored = category {
        info!("Ignoring notification: {:?}", &dto);
        return Ok((Status::Ok, "Notification is filtered"));
    }

    let mut tg_message_text = String::new();
    if let Some(title) = dto.title {
        tg_message_text.push_str(&title);
        tg_message_text.push_str("\n");
    }

    tg_message_text.push_str(&dto.text.unwrap());
    tg_message_text.push_str("\n---\n");
    tg_message_text.push_str(&app_name);

    tg.send_notification(chat_id, &tg_message_text).await?;
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
        app_config.telegram_bot_token.clone()
    );

    let mut config = rocket::Config::default();
    config.port = 8080;
    config.address = IpAddr::from_str("0.0.0.0").unwrap();

    let figment = Figment::from(config)
        .merge(Env::prefixed("ROCKET_").global());

    rocket::custom(figment)
        .mount("/", routes![handle_sms, handle_notification])
        .manage(telegram_client)
        .manage(app_config)
}
