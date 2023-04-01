use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    Message { message: String },
}

impl NetworkMessage {
    pub fn message(message: String) -> Self {
        Self::Message { message }
    }
}
