use connection::Connection;
use std::net::SocketAddr;

pub struct ClientId(usize);

pub struct ClientHandle {
    id: ClientId,
    ip: SocketAddr,
}

struct Client {
    tcp: Connection,
}
