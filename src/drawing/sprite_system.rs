use crate::{core::error::DataError, io::file_system::FileSystem};

use super::font::Font;

pub struct SpriteSystem {

}

impl SpriteSystem {
    pub fn new() -> Self {
        SpriteSystem { }
    }

    pub fn load_font(&self, file_system: &FileSystem, path: &str) -> Result<Font, DataError> {
        Ok(Font { })
    }
}
