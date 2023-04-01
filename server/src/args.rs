#[derive(clap::Parser)]
pub struct Args {
    /// An address
    #[arg(short = 'p', long = "port")]
    pub port: u16,
}
