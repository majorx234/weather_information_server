use crossbeam_queue::ArrayQueue;
use std::sync::{Arc, Mutex};

pub struct ServerElements {
    image_data: f32,
    arc_aq: Option<Arc<ArrayQueue<f32>>>,
}

impl ServerElements {
    pub fn new(arc_aq: Arc<ArrayQueue<f32>>) -> ServerElements {
        ServerElements {
            image_data: 0.0,
            arc_aq: Some(arc_aq),
        }
    }
    pub fn get_image_data(&mut self) -> f32 {
        if let Some(arc_aq) = &self.arc_aq {
            if let Some(image_data) = arc_aq.pop() {
                self.image_data = image_data;
            }
        }
        self.image_data
    }
}

impl Default for ServerElements {
    fn default() -> ServerElements {
        ServerElements {
            image_data: 0.0,
            arc_aq: None,
        }
    }
}

pub type ServerState = Arc<Mutex<ServerElements>>;
