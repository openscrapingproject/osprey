use serde::Deserialize;
use std::net::{Ipv4Addr};

fn default_address() -> Ipv4Addr {
    "127.0.0.1".parse().unwrap()
}

fn default_port() -> u16 {
    8080
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub debug: bool,

    #[serde(default = "default_address")]
    pub address: Ipv4Addr, // wow this works

    #[serde(default = "default_port")]
    pub port: u16,
}
