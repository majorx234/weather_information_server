use url::Url;
use weather_information::{config::Config, page_scraper::PageScraper};

fn main() {
    let url_plain = Config::new().get_weather_url();

    let weather_page = PageScraper::new();
    if let Ok(temperatur_data) = weather_page.extract_temperature_from(&url_plain.to_string()) {
        println!("it's {}°C", temperatur_data);
    }
}
