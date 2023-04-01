use connection::Connection;
use std::net::SocketAddr;

pub struct ClientId(pub usize);

pub struct ClientHandle {
    id: ClientId,
    ip: SocketAddr,
}

struct Client {
    tcp: Connection,
}
