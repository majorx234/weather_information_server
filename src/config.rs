use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

pub struct Config {
    host_ip: String,
    port: u32,
    scrap_freq: Duration,
    weather_url: String,
    selector: String,
}

impl Config {
    /// Config Constructor, reads env variables and sets config
    pub fn new() -> Config {
        let host_ip = std::env::var("HOST_IP").expect("HOST_IP not set");
        let port = std::env::var("PORT").expect("PORT not set");
        let weath = std::env::var("HOST_IP").expect("HOST_IP not set");
        let url = std::env::var("WEATHER_URL").expect("WEATHER_URL not set");
        let scrap_freq = std::time::Duration::from_secs(
            std::env::var("SCRAP_FREQ")
                .expect("SCRAP_FREQ not set")
                .parse::<u64>()
                .unwrap(),
        );

        let selector = std::env::var("SELECTOR").expect("SELECTOR not set");

        Config {
            host_ip,
            port: port.parse::<u32>().unwrap(),
            scrap_freq,
            weather_url: url,
            selector,
        }
    }
    pub fn get_host_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host_ip, self.port)[..]).unwrap()
    }
    pub fn get_scrap_frequency(&self) -> Duration {
        self.scrap_freq
    }
    pub fn get_app_user_agent(&self) -> String {
        "weather_app".to_string()
    }
    pub fn get_weather_url(&self) -> String {
        self.weather_url.clone()
    }
    pub fn get_selector(&self) -> String {
        self.selector.clone()
    }
}
