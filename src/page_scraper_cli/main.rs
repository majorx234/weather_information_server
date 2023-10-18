use weather_information::{config::Config, page_scraper::PageScraper};

fn main() {
    let config = Config::new();
    let url_plain = config.get_weather_url();

    let weather_page = PageScraper::new(&url_plain.to_string(), &config.get_selector());
    if let Ok(temperatur_data) = weather_page.extract_temperature_from() {
        println!("it's {}°C", temperatur_data);
    }
}
