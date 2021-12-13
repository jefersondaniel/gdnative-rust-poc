use std::rc::Rc;

use crate::core::error::DataError;
use super::mugen_font::MugenFont;
use super::{sff::{image::Palette, sff_parser}, sprite_file::SpriteFile};

pub struct SpriteSystem {}

impl SpriteSystem {
    pub fn new() -> Self {
        SpriteSystem { }
    }

    pub fn load_font(&self, path: &str) -> Result<MugenFont, DataError> {
        if path.to_lowercase().ends_with(".fnt") {
            return MugenFont::load_font_v1(path);
        } else if path.to_lowercase().ends_with(".def") {
            return MugenFont::load_font_v2(path, self);
        }

        Err(DataError::new(format!("Font file not supported: {}", path)))
    }

    pub fn get_sprite_file(&self, path: &str) -> Result<SpriteFile, DataError> {
        SpriteFile::load(path)
    }

    pub fn load_palettes(&self, path: &str) -> Result<Vec<Rc<Palette>>, DataError> {
        sff_parser::read_palettes(&path)
    }

    pub fn load_palette(&self, path: &str) -> Result<Rc<Palette>, DataError> {
        sff_parser::read_palette(&path)
    }
}
