use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SmsMessagesDto {
    pub device_id: String,
    pub carrier_name: String,
    pub messages: Vec<SmsMessageDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SmsMessageDto {
    pub from: String,
    pub message: String,
    pub received_at: u64
}

#[cfg(test)]
mod tests {
    use crate::dto::SmsMessagesDto;

    #[test]
    fn test_deserialization() {
        let json = "{\"deviceId\": \"8200195eb5e476d0\", \"carrierName\": \"MGTS\", \"messages\": [{\"from\": \"MTS-Bank\", \"message\": \"Oplata 719,92 RUB OKEY Ostatok: 253 373,18 RUB; *0530 \", \"receivedAt\": 1690554221000}]}";
        let dto: SmsMessagesDto = serde_json::from_str(json).unwrap();
        assert_eq!(dto.device_id, "8200195eb5e476d0");
        assert_eq!(dto.carrier_name, "MGTS");
    }
}