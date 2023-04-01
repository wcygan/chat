use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    Message { message: String },
    Shutdown,
    Heartbeat,
    Ack,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ToServer {
    Message { message: String },
    Join { name: String },
    Leave,
    KeepAlive,
}
