use std::rc::Rc;

use gdnative::core_types::Rect2;

use crate::{core::error::DataError, io::file_system::FileSystem, systems::visual_server::text::{font::Font, common::{FontSpacing, GlyphSpacing}, bitmap_font::BitmapFont}};

use super::{sff::{image::Palette, sff_parser}, sprite_file::SpriteFile, fnt_parser::read_fnt_file};

pub struct SpriteSystem {

}

impl SpriteSystem {
    pub fn new() -> Self {
        SpriteSystem { }
    }

    pub fn load_font(&self, path: &str) -> Result<Font, DataError> {
        if path.to_lowercase().ends_with(".fnt") {
            let fnt_file = read_fnt_file(path)?;

            let mut bitmap_font = BitmapFont::new(
                fnt_file.textures,
                FontSpacing {
                    line_gap: fnt_file.spacing.y,
                    ..Default::default()
                }
            );

            for (character, char_data) in fnt_file.char_map.iter() {
                bitmap_font.add_character(
                    *character,
                    0,
                    char_data.rect,
                    GlyphSpacing {
                        h_advance: char_data.rect.size.width + fnt_file.spacing.x,
                        ..Default::default()
                    }
                )
            }

            bitmap_font.add_character(
                ' ',
                0,
                Rect2::default(),
                GlyphSpacing {
                    h_advance: fnt_file.size.width + fnt_file.spacing.x,
                    ..Default::default()
                }
            );

            return Ok(Font::BitmapFont {
                font: bitmap_font,
            })
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
