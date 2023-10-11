use std::net::SocketAddr;
use std::str::FromStr;

pub struct Config {
    pub host_ip: String,
    pub port: u32,
}

impl Config {
    /// Config Constructor, reads env variables and sets config
    pub fn new() -> Config {
        let host_ip = std::env::var("HOST_IP").expect("HOST_IP not set");
        let port = std::env::var("PORT").expect("PORT not set");
        Config {
            host_ip,
            port: port.parse::<u32>().unwrap(),
        }
    }
    pub fn get_host_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host_ip, self.port)[..]).unwrap()
    }
}
