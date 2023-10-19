use crate::{config::Config, error::Error};
use reqwest;
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};

pub struct PageScraper {
    client: Client,
    url: Url,
    selector: Selector,
    data_index: usize,
}

impl PageScraper {
    pub fn new(url: &str, selector_str: &str, data_index: usize) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/geo+json".parse().unwrap());
        let client = reqwest::blocking::Client::builder()
            .user_agent(Config::new().get_app_user_agent())
            .default_headers(headers)
            .build()
            .expect("couldn't create requests client");
        let url = Url::parse(url).unwrap();
        let selector = Selector::parse(selector_str).unwrap();
        PageScraper {
            client,
            url,
            selector,
            data_index,
        }
    }

    pub fn extract_temperature_from(&self) -> Result<f32, Error> {
        let res = self.client.get(self.url.clone()).send()?.text()?;
        let document = Html::parse_document(res.as_str());

        let measurements = document
            .select(&self.selector)
            .map(|x| x.inner_html())
            .collect::<Vec<String>>();
        let s_len = measurements[self.data_index].len();
        let temp_string = measurements[2].replace("°C", "");
        let temperatur = temp_string.parse::<f32>().unwrap();
        Ok(temperatur)
    }
}
