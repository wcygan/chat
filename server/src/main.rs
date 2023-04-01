use crate::server::Listener;
use anyhow::Result;
use clap::Parser;
use tokio::select;

mod args;
mod client;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let shutdown = tokio_utils::ShutdownController::new();
    let mut listener = Listener::new(&shutdown, args).await?;

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl-C received, shutting down");
            shutdown.shutdown().await;
        }
        _ = tokio::spawn(async move { listener.listen().await }) => {
            println!("server listener shutting down")
        }
    }

    Ok(())
}
