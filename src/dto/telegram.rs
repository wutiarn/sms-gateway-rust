use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize)]
pub struct SendMessageRequest<'a> {
    pub text: &'a str,
    pub chat_id: &'a str
}

#[derive(Deserialize, Debug)]
pub struct SendMessageResponse {
    pub ok: bool,
    pub result: SendMessageResponseResult
}

#[derive(Deserialize, Debug)]
pub struct SendMessageResponseResult {
    pub message_id: u64
}