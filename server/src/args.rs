#[derive(clap::Parser)]
/// Spawn a chat server on the given port
pub struct Args {
    /// The port to use for the server
    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,
}
