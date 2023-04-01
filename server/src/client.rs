use crate::server::ServerHandle;
use common::message::FromServer;
use connection::Connection;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClientId(pub usize);

/// A handle to this actor, used by the server.
#[derive(Debug)]
pub struct ClientHandle {
    pub id: ClientId,
    ip: SocketAddr,
    chan: Sender<FromServer>,
    kill: JoinHandle<()>,
}

impl Drop for ClientHandle {
    fn drop(&mut self) {
        self.kill.abort()
    }
}

pub struct Client {
    pub ip: SocketAddr,
    pub id: ClientId,
    pub handle: ServerHandle,
    pub tcp: Connection,
}

impl Client {
    pub async fn run(&mut self) {
        todo!()
    }
}
