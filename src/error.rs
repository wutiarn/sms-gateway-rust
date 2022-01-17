use std::io::Cursor;
use anyhow::{anyhow, Error};
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;

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
}

impl From<anyhow::Error> for AppError {
    fn from(e: Error) -> Self {
        AppError::new(e)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, _: &'r Request) -> rocket::response::Result<'o> {
        let response_body = format!("{}: {}", self.status_code.to_string(), self.error.to_string());
        Response::build()
            .status(self.status_code)
            .sized_body(response_body.len(), Cursor::new(response_body))
            .ok()
    }
}