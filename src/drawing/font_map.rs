use std::collections::HashMap;

use crate::systems::visual_server::text::font::Font;

#[derive(Clone)]
pub struct FontMap {
    map: HashMap<usize, Font>,
}

impl FontMap {
    pub fn new(map: HashMap<usize, Font>) -> Self {
        FontMap { map }
    }
}
