use crate::internal;
use crate::server::ServerHandle;
use anyhow::Result;
use common::message::NetworkMessage;
use connection::Connection;
use std::io;
use std::net::SocketAddr;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClientId(pub usize);

/// A handle to this actor, used by the server.
#[derive(Debug)]
pub struct ClientHandle {
    pub id: ClientId,
    ip: SocketAddr,
    chan: Sender<NetworkMessage>,
    kill: JoinHandle<()>,
}

impl ClientHandle {
    pub fn send(&mut self, msg: NetworkMessage) -> Result<()> {
        if self.chan.try_send(msg).is_err() {
            Err(io::Error::new(io::ErrorKind::Other, "Client has shut down.").into())
        } else {
            Ok(())
        }
    }
}

impl Drop for ClientHandle {
    fn drop(&mut self) {
        self.kill.abort()
    }
}

pub struct ClientInfo {
    pub ip: SocketAddr,
    pub id: ClientId,
    pub server: ServerHandle,
    pub tcp: Connection,
}

struct ClientData {
    id: ClientId,
    server: ServerHandle,
    recv: Receiver<NetworkMessage>,
    conn: Connection,
}

/// Spawn a new client actor.
pub fn spawn_client(info: ClientInfo) {
    let (send, recv) = channel(64);

    let data = ClientData {
        id: info.id,
        server: info.server.clone(),
        conn: info.tcp,
        recv,
    };

    let (my_send, my_recv) = oneshot::channel();
    let kill = tokio::spawn(start_client(my_recv, data));

    let handle = ClientHandle {
        id: info.id,
        ip: info.ip,
        chan: send,
        kill,
    };

    let _ = my_send.send(handle);
}

async fn start_client(my_handle: oneshot::Receiver<ClientHandle>, mut data: ClientData) {
    let my_handle = match my_handle.await {
        Ok(my_handle) => my_handle,
        Err(_) => return,
    };

    data.server.send(internal::ToServer::join(my_handle)).await;

    let res = client_loop(data).await;
    match res {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Something went wrong: {}.", err);
        }
    }
}

/// This method performs the actual job of running the client actor.
async fn client_loop(mut data: ClientData) -> Result<()> {
    loop {
        tokio::select! {
            msg = data.recv.recv() => {
                if let Some(msg) = msg {
                    data.conn.write::<NetworkMessage>(&msg).await?;
                }
            }
            msg = data.conn.read::<NetworkMessage>() => {
                if let Ok(Some(msg)) = msg {
                    data.server.send(internal(data.id, msg)).await;
                }
            }
        }
    }
}

fn internal(id: ClientId, msg: NetworkMessage) -> internal::ToServer {
    match msg {
        NetworkMessage::Message { message } => internal::ToServer::message(id, message),
    }
}
