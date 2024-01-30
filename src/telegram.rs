use log::info;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;

use crate::dto::telegram::{SendMessageRequest, SendMessageResponse};

pub struct TelegramClient {
    bot_api_token: String,
    http_client: Client,
}

impl TelegramClient {
    pub fn new(bot_api_token: String) -> Self {
        let client = Client::builder().build().unwrap();
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
        let response = self.http_client.post(format!("https://api.telegram.org/bot{}/sendMessage", self.bot_api_token))
            .header(CONTENT_TYPE, "application/json")
            .json(&dto)
            .send().await?;

        let response_dto: SendMessageResponse = response.json().await?;
        info!("Sent message id: {}", response_dto.result.message_id);

        Ok(())
    }
}
