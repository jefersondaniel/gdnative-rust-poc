use std::collections::HashMap;

use super::font::Font;

#[derive(Clone)]
pub struct FontMap {
    map: HashMap<usize, Font>,
}

impl FontMap {
    pub fn new(map: HashMap<usize, Font>) -> Self {
        FontMap { map }
    }
}
