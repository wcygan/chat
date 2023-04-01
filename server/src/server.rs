use crate::client::{ClientHandle, ClientId};
use anyhow::Result;

use crate::internal;
use common::message::NetworkMessage;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_utils::{ShutdownController, ShutdownMonitor};

/// This struct is used by client actors to send messages to the server
#[derive(Clone, Debug)]
pub struct ServerHandle {
    chan: Sender<internal::ToServer>,
    next_id: Arc<AtomicUsize>,
}

impl ServerHandle {
    pub async fn send(&mut self, msg: internal::ToServer) {
        if self.chan.send(msg).await.is_err() {
            panic!("Main loop has shut down.");
        }
    }

    pub fn next_id(&self) -> ClientId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        ClientId(id)
    }
}

pub fn spawn_main_loop(shutdown: &ShutdownController) -> (ServerHandle, JoinHandle<()>) {
    let (send, recv) = channel(64);

    let handle = ServerHandle {
        chan: send,
        next_id: Default::default(),
    };

    let join = tokio::spawn({
        let monitor = shutdown.subscribe();
        async move {
            let res = main_loop(recv, monitor).await;
            match res {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Oops {}.", err);
                }
            }
        }
    });

    (handle, join)
}

#[derive(Default, Debug)]
struct Data {
    clients: HashMap<ClientId, ClientHandle>,
}

pub async fn main_loop(
    mut recv: Receiver<internal::ToServer>,
    mut monitor: ShutdownMonitor,
) -> Result<()> {
    let mut data = Data::default();

    while !monitor.is_shutdown() {
        select! {
            next = recv.recv() => {
                if let Some(msg) = next {
                    match msg {
                        internal::ToServer::Join{ client } => {
                            data.clients.insert(client.id, client);
                        },

                        internal::ToServer::Message{ client_id, message} => {
                            let m = NetworkMessage::message(message);
                            let mut to_remove = Vec::new();
                            for (id, client) in data.clients.iter_mut() {
                                if *id != client_id {
                                    if let Err(err) = client.send(m.clone()).await {
                                        to_remove.push(*id);
                                    }
                                }
                            }

                            for id in to_remove {
                                data.clients.remove(&id);
                            }
                        },
                        internal::ToServer::FatalError(err) => {

                            for (_, client) in data.clients.iter_mut() {
                                println!("Shutting down client");
                                let _ = client.send(NetworkMessage::Shutdown);
                            }

                            return Err(anyhow::anyhow!("{}", err));
                        },
                    }
                }
            },
            _ = monitor.recv() => {
                println!("Shutting down server");

                for (_, client) in data.clients.iter_mut() {
                    println!("Shutting down client");
                    match client.send(NetworkMessage::Shutdown).await {
                        Ok(()) => {}
                        Err(err) => {
                            println!("Error shutting down client: {}", err);
                        }
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                println!("Server shutdown complete.");

                return Ok(());
            },
        }
    }

    Ok(())
}
