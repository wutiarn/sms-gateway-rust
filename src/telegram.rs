pub struct TelegramClient {
    pub bot_api_token: String
}

impl TelegramClient {
    pub fn send_notification(&self, message: &str) {
        println!("Sending message: {}", message)
    }
}
