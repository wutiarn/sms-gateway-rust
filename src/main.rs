#[macro_use]
extern crate rocket;

use std::error::Error;

use rocket::futures::future::err;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::Accepted;

use dto::SmsMessageDto;

mod dto;

#[post("/hooks/sms", data = "<message>")]
fn index(message: SmsMessageDto) -> Result<&'static str, Custom<&'static str>> {
    println!("Body is: {:?}", message);
    if !message.validate_secret("131ba000-393d-4dea-a5d5-68e6558c0c68") {
        return Err(Custom(Status::Forbidden, "Token is incorrect"));
    }
    Ok("OK")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}