use crate::config::Config;
use reqwest;
use reqwest::blocking::Client;

struct PageScraper {
    client: Client,
}

impl PageScraper {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/geo+json".parse().unwrap());
        let client = reqwest::blocking::Client::builder()
            .user_agent(Config::new().get_app_user_agent())
            .default_headers(headers)
            .build()
            .expect("couldn't create requests client");
        PageScraper { client }
    }
}
