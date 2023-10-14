use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

pub struct Config {
    pub host_ip: String,
    pub port: u32,
    pub scrap_freq: Duration,
}

impl Config {
    /// Config Constructor, reads env variables and sets config
    pub fn new() -> Config {
        let host_ip = std::env::var("HOST_IP").expect("HOST_IP not set");
        let port = std::env::var("PORT").expect("PORT not set");
        let scrap_freq = std::time::Duration::from_secs(
            std::env::var("SCRAP_FREQ")
                .expect("SCRAP_FREQ not set")
                .parse::<u64>()
                .unwrap(),
        );

        Config {
            host_ip,
            port: port.parse::<u32>().unwrap(),
            scrap_freq,
        }
    }
    pub fn get_host_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host_ip, self.port)[..]).unwrap()
    }
    pub fn get_scrap_frequency(&self) -> Duration {
        self.scrap_freq
    }
}
