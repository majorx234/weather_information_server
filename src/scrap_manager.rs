use crate::config::Config;
use crate::page_scraper::PageScraper;
use crossbeam_queue::ArrayQueue;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_scraping(arc_aq: Arc<ArrayQueue<f32>>) {
    let scrapper_thread = thread::spawn(move || {
        let config = Config::new();
        let sec = config.get_scrap_frequency();
        let url_plain = config.get_weather_url();
        while true {
            std::thread::sleep(sec);
            let weather_page = PageScraper::new(
                &url_plain.to_string(),
                &config.get_selector(),
                config.get_data_index(),
            );
            if let Ok(temperatur_data) = weather_page.extract_temperature_from() {
                let _ = arc_aq.force_push(temperatur_data);
            }
        }
    });
}
