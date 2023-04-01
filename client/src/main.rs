use crate::args::Args;
use crate::client::Client;
use anyhow::Result;
use clap::Parser;
use connection::Connection;
use tokio::select;

mod args;
mod client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::dial(args.address).await?;
    let shutdown = tokio_utils::ShutdownController::new();
    let mut client = Client::new(conn, &shutdown);

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("ctrl-c received");
            shutdown.shutdown().await;
        }
        _ = tokio::spawn(async move { client.process().await; }) => {
            println!("client process finished");
        }
    }

    Ok(())
}
