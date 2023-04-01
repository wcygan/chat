use crate::args::Args;
use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc;
use tokio_utils::{ShutdownController, ShutdownMonitor};

pub struct Listener {
    listener: TcpListener,
    monitor: ShutdownMonitor,
    chan: mpsc::Sender<(TcpStream, SocketAddr)>,
}

struct Processor {
    monitor: ShutdownMonitor,
    chan: mpsc::Receiver<(TcpStream, SocketAddr)>,
}

impl Listener {
    pub async fn new(controller: &ShutdownController, args: Args) -> Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;

        let (tx, rx) = mpsc::channel(100);

        let mut processor = Processor {
            monitor: controller.subscribe(),
            chan: rx,
        };

        tokio::spawn(async move { processor.run().await });

        Ok(Self {
            listener,
            monitor: controller.subscribe(),
            chan: tx,
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

impl Processor {
    async fn run(&mut self) {
        while !self.monitor.is_shutdown() {
            select! {
                _ = self.monitor.recv() => {
                    println!("server processor shutting down");
                    return;
                }
                res = self.chan.recv() => {
                    if let Some((socket, addr)) = res {
                        println!("Received connection from: {}", addr);
                    }
                }
            }
        }
    }
}
