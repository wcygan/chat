use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio_utils::{ShutdownController, ShutdownMonitor};

pub struct Listener {
    monitor: ShutdownMonitor,
}

struct Processor {
    monitor: ShutdownMonitor,
}
