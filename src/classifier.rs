use std::ops::Deref;
use crate::dto::notification::NotificationDto;
use crate::dto::sms_message::SmsMessageDto;
use crate::classifier::MessageCategory::{Ignored, Normal};

pub enum MessageCategory {
    Normal,
    Ignored,
}

pub fn get_sms_category(msg: &SmsMessageDto) -> MessageCategory {
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

pub fn get_notification_category(msg: &NotificationDto) -> MessageCategory {
    match msg.package_name.deref() {
        "com.idamob.tinkoff.android" => {
            match &msg.text {
                None => {}
                Some(text) => {
                    if text.contains("Обновляется база номеров") {
                        return Ignored;
                    }
                }
            }
        }
        _ => {}
    }
    return Normal;
}
