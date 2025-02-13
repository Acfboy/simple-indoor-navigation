mod map;
mod guidance;

use map::{Map, Mark};
use guidance::Route;

#[derive(Default)]
pub struct Navigator {
    map: Map,
    pub route: Route,
}

impl Navigator {
    pub fn init(&mut self, context: &str) {
        self.map = serde_json::from_str(context).expect("Deserialize map");
    }

    pub fn navigate(&self, start: Mark, finish: Mark) {
        
    }
}
