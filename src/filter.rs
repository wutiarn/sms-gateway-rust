use std::ops::Deref;
use crate::dto::classifier::NotificationDto;
use crate::dto::sms_message::SmsMessageDto;
use crate::filter::MessageCategory::{Ignored, Normal};

pub enum MessageCategory {
    Normal,
    Ignored
}

pub fn get_sms_category(msg: &SmsMessageDto) -> MessageCategory{
    match msg.from.deref() {
        "Sbermarket" => {
            if !msg.message.contains("код для входа в профиль") {
                return Ignored;
            }
        }
        _ => {}
    }
    return Normal;
}

pub fn get_notification_category(msg: &NotificationDto) -> MessageCategory{
    return Normal;
}
