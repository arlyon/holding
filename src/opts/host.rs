use std::net::IpAddr;

use clap::Clap;

/// Hosts an existing world, allowing players to interact with it.
#[derive(Clap)]
struct Host {
    /// The host to bind to.
    #[clap(short, default_value = "0.0.0.0")]
    host: IpAddr,

    /// The port to bind to.
    #[clap(short, default_value = "8080")]
    port: String,
}
