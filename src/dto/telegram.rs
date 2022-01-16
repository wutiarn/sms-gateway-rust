use serde::Serialize;

#[derive(Serialize)]
struct SendMessageRequest {
    text: String,
    chat_id: String
}
