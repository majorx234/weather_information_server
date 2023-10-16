use crate::{config::Config, error::Error};
use reqwest;
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};

pub struct PageScraper {
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

    pub fn extract_temperature_from(&self, url: &str, selector: &str) -> Result<f32, Error> {
        let url = Url::parse(url)?;
        let res = self.client.get(url).send()?.text()?;
        let document = Html::parse_document(res.as_str());

        let measurements_selector = Selector::parse(selector).unwrap();
        let measurements = document
            .select(&measurements_selector)
            .map(|x| x.inner_html())
            .collect::<Vec<String>>();
        let s_len = measurements[2].len();
        let temp_string = measurements[2].replace("°C", "");
        let temperatur = temp_string.parse::<f32>().unwrap();
        Ok(temperatur)
    }
}
