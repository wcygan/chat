use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum FromServer {
    Message { message: String },
    Shutdown,
    Heartbeat,
    Ack,
}

#[derive(Serialize, Deserialize)]
pub enum ToServer {
    Message { message: String },
    Join { name: String },
    Leave,
    KeepAlive,
}
