use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub(crate) enum BackendMessageType {
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

#[derive(Serialize, Clone)]
pub(crate) struct BackendMessage {
    pub(crate) message_type: BackendMessageType,
    pub(crate) data: String,
    pub(crate) id: Option<Uuid>,
}