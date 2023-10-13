use std::boxed::Box;
use std::sync::Arc;

pub struct ServerElements {
    image_data: u32,
    rx_image_channel: Option<crossbeam_channel::Receiver<u32>>,
}

impl ServerElements {
    pub fn new(rx_image_channel: crossbeam_channel::Receiver<u32>) -> ServerElements {
        ServerElements {
            image_data: 0,
            rx_image_channel: Some(rx_image_channel),
        }
    }
    pub fn get_image_data(&mut self) -> u32 {
        if let Some(rx_image_channel) = &self.rx_image_channel {
            match rx_image_channel.recv() {
                Ok(image_data) => {
                    self.image_data = image_data;
                }
                Err(_) => (),
            }
        }
        return self.image_data;
    }
}

impl Default for ServerElements {
    fn default() -> ServerElements {
        ServerElements {
            image_data: 0,
            rx_image_channel: None,
        }
    }
}

pub type ServerState = Arc<ServerElements>;
