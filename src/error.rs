use std::io::Cursor;
use anyhow::{Error};
use rocket::{Request, Response};
use rocket::http::{Header, Status};
use rocket::response::Responder;
use log::info;

pub struct AppError {
    error: anyhow::Error,
    status_code: Status,
}

impl AppError {
    pub fn new(error: anyhow::Error) -> Self {
        AppError {
            error,
            status_code: Status::InternalServerError
        }
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status_code = status;
        self
    }

    pub fn err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: Error) -> Self {
        AppError::new(e)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, _: &'r Request) -> rocket::response::Result<'o> {
        let response_body = format!("{}: {}", self.status_code.to_string(), self.error.to_string());
        info!("Responding with error: {}", response_body);
        Response::build()
            .status(self.status_code)
            .header(Header::new("Content-Type", "text/plain"))
            .sized_body(response_body.len(), Cursor::new(response_body))
            .ok()
    }
}