use std::{sync::Arc, collections::HashMap};

use ab_glyph::{FontArc, FontVec, InvalidFont, Font, GlyphId};
use gdnative::{core_types::{ByteArray, Size2}, api::{Image, visual_server::TextureFlags}};

use crate::systems::visual_server::texture::Texture;

use super::common::FontSpacing;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CacheKey(GlyphId, i32);

pub struct VectorFont {
    font: FontArc,
    texture_cache: HashMap<CacheKey, Arc<Texture>>
}

impl VectorFont {
    pub fn try_from_bytes(font_data: Vec<u8>) -> Result<Self, InvalidFont> {
        gdnative::godot_print!("Loaded font with {} bytes", font_data.len());

        let font = FontVec::try_from_vec(font_data)?;
        let font = FontArc::new(font);

        Ok(VectorFont { font, texture_cache: HashMap::new() })
    }

    pub fn get_spacing(&self, scale: i32) -> FontSpacing {
        FontSpacing {
            top: 0.0,
            bottom: 0.0,
            character: 0.0,
            space: 0.0,
        }
    }

    pub fn get_char_size(
        &self,
        glyph: char,
        scale: i32,
    ) -> Size2 {
        let glyph_id = self.font.glyph_id(glyph);
        let glyph = glyph_id.with_scale(scale as f32);

        if let Some(q) = self.font.outline_glyph(glyph) {
            let bounds = q.px_bounds();
            let width = bounds.width() as usize;
            let height = bounds.height() as usize;

            return Size2::new(width as f32, height as f32);
        }

        Size2::new(0.0, 0.0)
    }

    pub fn get_texture(
        &mut self,
        character: char,
        scale: i32,
    ) -> Option<Arc<Texture>> {
        let glyph_id = self.font.glyph_id(character);
        let glyph = glyph_id.with_scale(scale as f32);
        let cache_key = CacheKey(glyph_id, scale);

        if self.texture_cache.contains_key(&cache_key) {
            let cache_result = self.texture_cache.get(&cache_key).unwrap();

            return Some(cache_result.clone())
        }

        if let Some(q) = self.font.outline_glyph(glyph) {
            let bounds = q.px_bounds();
            let width = bounds.width() as usize;
            let height = bounds.height() as usize;

            let mut alpha: Vec<f32> = vec![0.0; width * height];

            q.draw(|x, y, v| {
                alpha[y as usize * width  as usize + x as usize] = v;
            });

            let byte_array = alpha
                .iter()
                .map(|a| vec![255, 255, 255, (*a * 255.0) as u8])
                .flatten()
                .collect::<Vec<u8>>();

            let dest = ByteArray::from_slice(byte_array.as_slice());
            let image = Image::new();

            image.create_from_data(
                width as i64,
                height as i64,
                false,
                Image::FORMAT_RGBA8,
                dest,
            );

            let texture = Texture::allocate(image, TextureFlags::FLAGS_DEFAULT);

            self.texture_cache.insert(cache_key, texture.clone());

            return Some(texture)
        }

        None
    }
}
