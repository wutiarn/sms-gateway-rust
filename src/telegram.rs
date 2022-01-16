use std::error::Error;
use std::io::Read;
use hyper::{Body, Client, Method, Request};
use hyper::client::HttpConnector;
use hyper::header::CONTENT_TYPE;
use hyper_tls::HttpsConnector;
use rocket::futures::StreamExt;
use rocket::http::Header;
use crate::dto::telegram::{SendMessageRequest, SendMessageResponse};

pub struct TelegramClient {
    bot_api_token: String,
    recipient_id: String,
    http_client: Client<HttpsConnector<HttpConnector>>,
}

impl TelegramClient {
    pub fn new(bot_api_token: String, recipient_id: String) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build(https);

        TelegramClient {
            http_client: client,
            recipient_id,
            bot_api_token,
        }
    }
    pub async fn send_notification(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let dto = SendMessageRequest {
            text: &message,
            chat_id: &self.recipient_id
        };
        let dto_json = serde_json::to_string(&dto)?;
        let request = Request::builder()
            .method(Method::POST)
            .uri(format!("https://api.telegram.org/bot{}/sendMessage", self.bot_api_token))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(dto_json))?;

        let mut response = self.http_client.request(request).await?;
        let response_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let response_str = String::from_utf8(response_bytes.to_vec())?;
        let response_dto: SendMessageResponse = serde_json::from_slice(&response_bytes)?;

        println!("Telegram response: {}", response_str);
        println!("Sent message id: {}", response_dto.result.message_id);

        Ok(())
    }
}
