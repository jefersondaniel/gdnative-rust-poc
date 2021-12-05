use std::sync::Arc;

use gdnative::{core_types::Rect2};

use crate::systems::visual_server::texture::Texture;

use super::{vector_font::VectorFont, common::{GlyphSpacing, FontSpacing}};

#[derive(Clone)]
pub enum Font {
    None,
    VectorFont {
        font: VectorFont
    }
}

impl Default for Font {
    fn default() -> Self { Font::None }
}

impl Font {
    pub fn get_char_rect(&self, current: char, scale: i32) -> Rect2 {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_char_rect(
                    current,
                    scale
                )
            },
            Font::None => Rect2::default()
        }
    }

    pub fn get_glyph_spacing(&self, previous: Option<char>, current: char, scale: i32) -> GlyphSpacing {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_glyph_spacing(previous, current, scale)
            },
            Font::None => GlyphSpacing::default(),
        }
    }

    pub fn get_font_spacing(&self, scale: i32) -> FontSpacing {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_font_spacing(scale)
            },
            Font::None => FontSpacing::default(),
        }
    }

    pub fn get_texture(
        &mut self,
        glyph: char,
        scale: i32,
    ) -> Option<Arc<Texture>> {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_texture(glyph, scale)
            },
            Font::None => None
        }
    }
}
