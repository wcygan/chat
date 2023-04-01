use crate::client::{ClientHandle, ClientId};
use anyhow::Result;
use common::message::ToServer;
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
    chan: Sender<ToServer>,
    next_id: Arc<AtomicUsize>,
}

impl ServerHandle {
    pub async fn send(&mut self, msg: ToServer) {
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

async fn main_loop(mut recv: Receiver<ToServer>, mut monitor: ShutdownMonitor) -> Result<()> {
    let mut data = Data::default();

    while !monitor.is_shutdown() {
        select! {
            next = recv.recv() => {
                if let Some(msg) = next {
                    match msg {
                        _ => todo!("implement the client actions (join, message, leave, etc")
                    }
                }
            },
            _ = monitor.recv() => {
                eprintln!("Main loop has shut down.");
                return Ok(());
            },
        }
    }

    Ok(())
}
