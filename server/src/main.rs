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
    let addr = format!("0.0.0.0:{}", args.port).parse()?;
    println!("listening on {}", addr);
    let shutdown = tokio_utils::ShutdownController::new();
    let (handle, join) = server::spawn_main_loop(&shutdown);

    select! {
        _ = accept::start_accept(addr, handle, &shutdown) => {}
        _ = join => {}
        _ = tokio::signal::ctrl_c() => {
            shutdown.shutdown().await;
        }
    }

    Ok(())
}
