use crate::args::Args;
use crate::client::ClientId;
use anyhow::Result;
use common::message::ToServer;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc;
use tokio_utils::{ShutdownController, ShutdownMonitor};

/// Listen for incoming connections
pub struct Listener {
    listener: TcpListener,
    monitor: ShutdownMonitor,
    accept_clients_tx: mpsc::Sender<(TcpStream, SocketAddr)>,
    send_client_message: mpsc::Sender<(ClientId, ToServer)>,
}

/// Accept incoming connections and spawn a new client actor for each one
struct Acceptor {
    monitor: ShutdownMonitor,
    accept_clients_rx: mpsc::Receiver<(TcpStream, SocketAddr)>,
    next_id: Arc<AtomicUsize>,
}

/// Process messages from clients
struct Processor {
    monitor: ShutdownMonitor,
    recv_client_message: mpsc::Receiver<(ClientId, ToServer)>,
}

impl Listener {
    pub async fn new(controller: &ShutdownController, args: Args) -> Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;

        let (send_client_message, recv_client_message) = mpsc::channel(100);
        let (accept_clients_tx, accept_clients_rx) = mpsc::channel(20);

        // Spawn the client acceptor
        let mut acceptor = Acceptor {
            monitor: controller.subscribe(),
            accept_clients_rx,
            next_id: Arc::new(AtomicUsize::new(0)),
        };

        tokio::spawn(async move { acceptor.run().await });

        // Spawn the client message processor
        let mut processor = Processor {
            monitor: controller.subscribe(),
            recv_client_message,
        };

        tokio::spawn(async move { processor.run().await });

        Ok(Self {
            listener,
            monitor: controller.subscribe(),
            accept_clients_tx,
            send_client_message,
        })
    }

    pub async fn listen(&mut self) {
        while !self.monitor.is_shutdown() {
            select! {
                _ = self.monitor.recv() => {
                    println!("server listener shutting down");
                    return;
                }
                res = self.listener.accept() => {
                    if let Ok((socket, addr)) = res {
                        println!("Accepted connection from: {}", addr);
                    }
                }
            }
        }
    }
}

impl Acceptor {
    async fn run(&mut self) {
        while !self.monitor.is_shutdown() {
            select! {
                _ = self.monitor.recv() => {
                    println!("server acceptor shutting down");
                    return;
                }
                res = self.accept_clients_rx.recv() => {
                    if let Some((socket, addr)) = res {
                        println!("Received connection from: {}", addr);
                    }
                }
            }
        }
    }

    pub fn next_id(&self) -> ClientId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        ClientId(id)
    }
}

impl Processor {
    async fn run(&mut self) {
        while !self.monitor.is_shutdown() {
            select! {
                _ = self.monitor.recv() => {
                    println!("server processor shutting down");
                    return;
                }
                res = self.recv_client_message.recv() => {
                    if let Some((id, msg)) = res {
                        println!("Received message from client: {:?}", msg);
                    }
                }
            }
        }
    }
}
