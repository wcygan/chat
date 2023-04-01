use crate::args::Args;
use anyhow::Result;
use clap::Parser;
use connection::Connection;

mod args;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::dial(args.address).await?;
    Ok(())
}
