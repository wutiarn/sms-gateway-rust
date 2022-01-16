use serde::Serialize;

#[derive(Serialize)]
pub struct SendMessageRequest<'a> {
    pub text: &'a str,
    pub chat_id: &'a str
}
