use lazy_static::lazy_static;
use rocket::data::FromData;
use serde::{Deserialize, Serialize};
use time::format_description;
use time::format_description::FormatItem;

const DELIMITER: &str = "^~";

lazy_static! {
    static ref DATETIME_FORMAT: Vec<FormatItem<'static>> = format_description::parse("[day].[month].[year] [hour].[minute]").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SmsMessagesDto {
    pub device_id: String,
    pub career_name: String,
    pub messages: Vec<SmsMessagesDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsMessageDto {
    pub from: String,
    pub message: String,
    pub received_at: u64
}
