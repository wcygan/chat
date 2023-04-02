#[derive(clap::Parser)]
/// A client used to connect to a chat server
pub struct Args {
    /// The address of the server to connect to
    #[arg(short = 'a', long = "address", default_value = "127.0.0.1:8080")]
    pub address: String,
}
