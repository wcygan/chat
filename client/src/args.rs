#[derive(clap::Parser)]
pub struct Args {
    /// An address
    #[arg(short = 'a', long = "address")]
    pub address: String,
}
