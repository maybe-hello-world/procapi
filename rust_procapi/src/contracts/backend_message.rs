use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
enum BackendMessageType {
    Short,
    Long,
}

impl BackendMessageType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            BackendMessageType::Short => "short",
            BackendMessageType::Long => "long"
        }
    }
}

#[derive(Serialize)]
struct BackendMessage {
    message_type: String,
    data: String,
    id: Uuid,
}