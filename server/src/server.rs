use crate::args::Args;
use anyhow::Result;
use clap::Parser;
use common::message::ToServer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc;
use tokio_utils::{ShutdownController, ShutdownMonitor};

pub struct Listener {
    listener: TcpListener,
    monitor: ShutdownMonitor,
    chan: mpsc::Sender<(ToServer, SocketAddr)>,
}

struct Processor {
    monitor: ShutdownMonitor,
    chan: mpsc::Receiver<(ToServer, SocketAddr)>,
}

impl Listener {
    pub async fn new(controller: &ShutdownController) -> Result<Self> {
        let args = Args::parse();
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
                    println!("server listener shutting down")
                }
            }
        }
    }
}

impl Processor {
    async fn run(&mut self) {
        while !self.monitor.is_shutdown() {
            if let Some((msg, addr)) = self.chan.recv().await {
                println!("Received message: {:?} from: {}", msg, addr);
            }
        }
    }
}
