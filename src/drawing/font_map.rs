use std::collections::HashMap;

use super::mugen_font::MugenFont;

#[derive(Clone)]
pub struct FontMap {
    map: HashMap<usize, MugenFont>,
}

impl FontMap {
    pub fn new(map: HashMap<usize, MugenFont>) -> Self {
        FontMap { map }
    }
}
