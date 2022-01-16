use std::error::Error;
use std::io::Read;
use hyper::{Body, Client, Method, Request};
use hyper::client::HttpConnector;
use hyper::header::CONTENT_TYPE;
use hyper_tls::HttpsConnector;
use rocket::http::Header;
use crate::dto::telegram::SendMessageRequest;

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
    pub async fn send_notification(&self, message: &str) -> Result<String, Box<dyn Error>> {
        println!("Sending message: {}", message);
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

        let response = self.http_client.request(request).await?;
        let response_body = hyper::body::to_bytes(response.into_body()).await?;
        println!("{:?}", response_body);
        Ok("Test".to_string())
    }
}
