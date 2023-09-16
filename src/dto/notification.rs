use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct NotificationDto {
    pub device_id: String,
    pub package_name: String,
    pub title: Option<String>,
    pub text: Option<String>
}
