use hyper::{Body, Client, Method, Request};
use hyper::client::HttpConnector;
use hyper::header::CONTENT_TYPE;
use hyper_tls::HttpsConnector;
use log::info;

use crate::dto::telegram::{SendMessageRequest, SendMessageResponse};

pub struct TelegramClient {
    bot_api_token: String,
    http_client: Client<HttpsConnector<HttpConnector>>,
}

impl TelegramClient {
    pub fn new(bot_api_token: String) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build(https);

        TelegramClient {
            http_client: client,
            bot_api_token,
        }
    }
    pub async fn send_notification(&self, chat_id: &str, message: &str) -> Result<(), anyhow::Error> {
        let dto = SendMessageRequest {
            text: message,
            chat_id,
        };
        let dto_json = serde_json::to_string(&dto)?;
        let request = Request::builder()
            .method(Method::POST)
            .uri(format!("https://api.telegram.org/bot{}/sendMessage", self.bot_api_token))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(dto_json))?;

        let response = self.http_client.request(request).await?;
        let resp_bytes = hyper::body::to_bytes(response).await?;
        // let response_str = String::from_utf8_lossy(&resp_bytes);
        // info!("Telegram response: {}", response_str);

        let response_dto: SendMessageResponse = serde_json::from_slice(&resp_bytes)?;
        info!("Sent message id: {}", response_dto.result.message_id);

        Ok(())
    }
}
