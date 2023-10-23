use crate::image_buffer::ImageBuffer;
use crossbeam_queue::ArrayQueue;
use std::sync::{Arc, Mutex};

pub struct ServerElements {
    image_data: ImageBuffer,
    arc_aq: Option<Arc<ArrayQueue<f32>>>,
}

impl ServerElements {
    pub fn new(arc_aq: Arc<ArrayQueue<f32>>) -> ServerElements {
        ServerElements {
            image_data: ImageBuffer::new(),
            arc_aq: Some(arc_aq),
        }
    }
    pub fn get_image_data(&mut self) -> Vec<u8> {
        if let Some(arc_aq) = &self.arc_aq {
            if let Some(image_data) = arc_aq.pop() {
                self.image_data.update_image(image_data);
            }
        }
        self.image_data.get_image()
    }
    pub fn get_eink_data(&mut self) -> Vec<u8> {
        if let Some(arc_aq) = &self.arc_aq {
            if let Some(image_data) = arc_aq.pop() {
                self.image_data.update_image(image_data);
            }
        }
        self.image_data.get_eink_buffer()
    }
}

impl Default for ServerElements {
    fn default() -> ServerElements {
        ServerElements {
            image_data: ImageBuffer::new(),
            arc_aq: None,
        }
    }
}

pub type ServerState = Arc<Mutex<ServerElements>>;
