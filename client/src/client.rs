use common::message::NetworkMessage;
use connection::Connection;
use tokio::select;
use tokio_utils::{ShutdownController, ShutdownMonitor};

pub struct Client {
    monitor: ShutdownMonitor,
    conn: Connection,
}

impl Client {
    pub fn new(conn: Connection, shutdown: &ShutdownController) -> Self {
        let monitor = shutdown.subscribe();
        Self { monitor, conn }
    }

    pub async fn process(&mut self) {
        let mut stdin = tokio_utils::recv_from_stdin(10);

        while !self.monitor.is_shutdown() {
            select! {
                _ = self.monitor.recv() => {
                    println!("client shutting down");
                }
                res = self.conn.read::<NetworkMessage>() => {
                    if let Ok(Some(msg)) = res {
                        match msg {
                            NetworkMessage::Message { message } => {
                                println!("{}", message);
                            }
                            NetworkMessage::Shutdown => {
                                println!("shutting down!");
                                return;
                            }
                        }
                    }
                }
                line = stdin.recv() => {
                    if let Some(s) = line {
                        let _ = self.conn.write::<NetworkMessage>(&NetworkMessage::Message { message: s }).await;
                    }
                }
            }
        }
    }
}
