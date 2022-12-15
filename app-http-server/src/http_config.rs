use app_macros::config;
use std::net::SocketAddr;

#[config]
#[derive(Debug)]
pub struct HttpConfig {
    pub address: SocketAddr,
    pub hot_reload: bool,
}
