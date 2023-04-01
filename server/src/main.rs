use anyhow::Result;
use clap::Parser;
use tokio::select;

mod accept;
mod args;
mod client;
mod internal;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let addr = format!("127.0.0.1:{}", args.port).parse()?;
    let shutdown = tokio_utils::ShutdownController::new();
    let (handle, join) = server::spawn_main_loop(&shutdown);

    tokio::spawn(async move {
        accept::start_accept(addr, handle).await;
    });

    select! {
        _ = join => {}
        _ = tokio::signal::ctrl_c() => {
            shutdown.shutdown().await;
        }
    }

    Ok(())
}
