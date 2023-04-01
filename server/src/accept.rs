use connection::Connection;
use std::io;
use std::net::SocketAddr;

use crate::server::ServerHandle;

use crate::client::{spawn_client, ClientInfo};
use crate::internal;

use tokio::net::TcpListener;
use tokio_utils::ShutdownController;

pub async fn start_accept(
    bind: SocketAddr,
    mut handle: ServerHandle,
    shutdown: &ShutdownController,
) {
    let res = accept_loop(bind, handle.clone(), shutdown).await;
    match res {
        Ok(()) => {}
        Err(err) => {
            handle
                .send(internal::ToServer::FatalError(err.to_string()))
                .await;
        }
    }
}

pub async fn accept_loop(
    bind: SocketAddr,
    server: ServerHandle,
    shutdown: &ShutdownController,
) -> Result<(), io::Error> {
    let listen = TcpListener::bind(bind).await?;

    loop {
        let (tcp, ip) = listen.accept().await?;
        println!("New connection from {}", ip);

        let id = server.next_id();

        let client = ClientInfo {
            shutdown: shutdown.subscribe(),
            ip,
            id,
            tcp: Connection::new(tcp),
            server: server.clone(),
        };

        spawn_client(client);
    }
}
