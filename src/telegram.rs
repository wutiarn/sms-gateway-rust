pub struct TelegramClient {}

impl TelegramClient {
    pub fn send_notification(&self, message: &str) {
        println!("Sending message: {}", message)
    }
}
