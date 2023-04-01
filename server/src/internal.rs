use crate::client::{ClientHandle, ClientId};

pub enum ToServer {
    Message {
        client_id: ClientId,
        message: String,
    },
    Join {
        client: ClientHandle,
    },
    FatalError(String),
}

impl ToServer {
    pub fn message(client_id: ClientId, message: String) -> Self {
        Self::Message { client_id, message }
    }

    pub fn join(client: ClientHandle) -> Self {
        Self::Join { client }
    }

    pub fn fatal_error(err: String) -> Self {
        Self::FatalError(err)
    }
}
