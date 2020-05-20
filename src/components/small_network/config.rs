use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// Small network configuration
pub struct Config {
    /// Interface to bind to. If it is the same as the in `root_addr`, attempt
    /// become the root node for this particular small network.
    pub bind_interface: IpAddr,

    /// Port to bind to when not the root node. Use 0 for a random port.
    pub bind_port: u16,

    /// Address to connect to join the network.
    pub root_addr: SocketAddr,

    /// Path to certificate file.
    pub cert: Option<PathBuf>,

    /// Path to private key for certificate.
    pub private_key: Option<PathBuf>,

    /// Maximum number of retries when trying to connect to an outgoing node. Unlimited if `None`.
    pub max_outgoing_retries: Option<u32>,
}

impl Config {
    /// Creates a default instance for `SmallNetwork` with a constant port.
    pub fn default_on_port(port: u16) -> Self {
        Config {
            bind_interface: Ipv4Addr::new(127, 0, 0, 1).into(),
            bind_port: 0,
            root_addr: (Ipv4Addr::new(127, 0, 0, 1), port).into(),
            cert: None,
            private_key: None,
            max_outgoing_retries: None,
        }
    }
}
