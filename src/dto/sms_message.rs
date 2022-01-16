use std::time::SystemTime;

use rocket::{Data, Request};
use rocket::data::{FromData, Outcome};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::tokio::io::AsyncReadExt;
use time::{Date, PrimitiveDateTime};
use time::error::Parse;
use time::format_description;
use time::format_description::FormatItem;
use time::parsing::Parsed;
use lazy_static::lazy_static;

const DELIMITER: &str = "^~";

lazy_static! {
    static ref DATETIME_FORMAT: Vec<FormatItem<'static>> = format_description::parse("[day].[month].[year] [hour].[minute]").unwrap();
}

#[derive(Debug)]
pub struct SmsMessageDto {
    secret: String,
    pub from: String,
    pub timestamp: PrimitiveDateTime,
    pub body: String,
}

impl SmsMessageDto {
    pub fn validate_secret(&self, expected: &str) -> bool {
        self.secret == expected
    }
}

#[derive(Debug)]
pub enum SmsMessageParseError {
    TooLarge,
    DateTimeParseFailed(time::error::Parse),
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for SmsMessageDto {
    type Error = SmsMessageParseError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let limit = req.limits().get("string").unwrap();
        let body_str = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, SmsMessageParseError::TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, SmsMessageParseError::Io(e)))
        };
        let parts: Vec<&str> = body_str.split(DELIMITER).collect();

        let datetime_str = format!("{} {}", parts[3], parts[4]);
        let datetime = match PrimitiveDateTime::parse(&datetime_str, &DATETIME_FORMAT) {
            Ok(it) => it,
            Err(e) => return Failure((Status::InternalServerError, SmsMessageParseError::DateTimeParseFailed(e)))
        };

        Success(
            SmsMessageDto {
                secret: parts[0].to_string(),
                from: parts[1].to_string(),
                body: parts[2].to_string(),
                timestamp: datetime,
            }
        )
    }
}