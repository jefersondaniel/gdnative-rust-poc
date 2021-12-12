use std::{sync::Arc, collections::HashMap};

use gdnative::{core_types::Rect2};

use crate::systems::visual_server::texture::Texture;

use super::{vector_font::{VectorFont, VectorFontCacheKey}, common::{GlyphSpacing, FontSpacing}, bitmap_font::BitmapFont};

#[derive(Clone)]
pub enum Font {
    None,
    VectorFont {
        font: VectorFont
    },
    BitmapFont {
        font: BitmapFont
    }
}

impl Default for Font {
    fn default() -> Self { Font::None }
}

impl Font {
    pub fn get_char_source_rect(&self, current: char, scale: i32) -> Rect2 {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_char_source_rect(
                    current,
                    scale
                )
            },
            Font::BitmapFont { font, .. } => {
                font.get_char_source_rect(current)
            },
            Font::None => Rect2::default()
        }
    }

    pub fn get_char_dest_rect(&self, current: char, scale: i32) -> Rect2 {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_char_dest_rect(
                    current,
                    scale
                )
            },
            Font::BitmapFont { font, .. } => {
                font.get_char_dest_rect(current)
            },
            Font::None => Rect2::default()
        }
    }

    pub fn get_glyph_spacing(&self, previous: Option<char>, current: char, scale: i32) -> GlyphSpacing {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_glyph_spacing(previous, current, scale)
            },
            Font::BitmapFont { font, .. } => {
                font.get_glyph_spacing(current)
            },
            Font::None => GlyphSpacing::default(),
        }
    }

    pub fn get_font_spacing(&self, scale: i32) -> FontSpacing {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_font_spacing(scale)
            },
            Font::BitmapFont { font, .. } => {
                font.get_font_spacing()
            },
            Font::None => FontSpacing::default(),
        }
    }

    pub fn get_texture(
        &self,
        glyph: char,
        scale: i32,
        vector_font_cache: &mut HashMap<VectorFontCacheKey, Arc<Texture>>,
    ) -> Option<Arc<Texture>> {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_texture(glyph, scale, vector_font_cache)
            },
            Font::BitmapFont { font, .. } => {
                font.get_texture(glyph)
            },
            Font::None => None
        }
    }
}
