use anyhow::Result;
use clap::Parser;
use tokio::select;

mod accept;
mod args;
mod client;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let shutdown = tokio_utils::ShutdownController::new();

    Ok(())
}
