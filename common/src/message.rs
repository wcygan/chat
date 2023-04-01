use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum FromServer {
    Message { message: String },
    Shutdown,
    Heartbeat,
    Ack,
}

#[derive(Serialize, Deserialize)]
enum ToServer {
    Message { message: String },
    Join { name: String },
    Leave,
    KeepAlive,
}
