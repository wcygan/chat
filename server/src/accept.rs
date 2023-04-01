use connection::Connection;
use std::io;
use std::net::SocketAddr;

use crate::server::ServerHandle;

use crate::client::Client;
use common::message::ToServer;
use tokio::net::TcpListener;

pub async fn start_accept(bind: SocketAddr, mut handle: ServerHandle) {
    let res = accept_loop(bind, handle.clone()).await;
    match res {
        Ok(()) => {}
        Err(err) => {
            handle.send(ToServer::FatalError(err.to_string())).await;
        }
    }
}

pub async fn accept_loop(bind: SocketAddr, handle: ServerHandle) -> Result<(), io::Error> {
    let listen = TcpListener::bind(bind).await?;

    loop {
        let (tcp, ip) = listen.accept().await?;

        let id = handle.next_id();

        let mut client = Client {
            ip,
            id,
            tcp: Connection::new(tcp),
            handle: handle.clone(),
        };

        tokio::spawn(async move {
            client.run().await;
        });
    }
}
