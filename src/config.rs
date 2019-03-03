use std::default::Default;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct DevproxyConfig {
    pub addr: SocketAddr,
}

impl Default for DevproxyConfig {
    fn default() -> Self {
        DevproxyConfig {
            addr: "127.0.0.1:8080".parse().unwrap(),
        }
    }
}
