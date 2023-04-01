use connection::Connection;
use std::io;
use std::net::SocketAddr;

use crate::server::ServerHandle;

use crate::client::{spawn_client, ClientInfo};
use crate::internal;
use common::message::NetworkMessage;
use tokio::net::TcpListener;

pub async fn start_accept(bind: SocketAddr, mut handle: ServerHandle) {
    let res = accept_loop(bind, handle.clone()).await;
    match res {
        Ok(()) => {}
        Err(err) => {
            handle
                .send(internal::ToServer::FatalError(err.to_string()))
                .await;
        }
    }
}

pub async fn accept_loop(bind: SocketAddr, server: ServerHandle) -> Result<(), io::Error> {
    let listen = TcpListener::bind(bind).await?;

    loop {
        let (tcp, ip) = listen.accept().await?;
        println!("New connection from {}", ip);

        let id = server.next_id();

        let mut client = ClientInfo {
            ip,
            id,
            tcp: Connection::new(tcp),
            server: server.clone(),
        };

        spawn_client(client);
    }
}
