use std::{collections::HashMap, sync::Arc};

use gdnative::core_types::{Rect2, Size2, Point2};

use crate::systems::visual_server::texture::Texture;

use super::common::{GlyphSpacing, FontSpacing};

#[derive(Clone)]
pub struct BitmapFont {
    textures: Vec<Arc<Texture>>,
    texture_map: HashMap<char, usize>,
    rect_map: HashMap<char, Rect2>,
    spacing_map: HashMap<char, GlyphSpacing>,
    spacing: FontSpacing
}

impl BitmapFont {
    pub fn new(textures: Vec<Arc<Texture>>, spacing: FontSpacing) -> Self {
        Self {
            textures,
            texture_map: HashMap::new(),
            rect_map: HashMap::new(),
            spacing_map: HashMap::new(),
            spacing
        }
    }

    pub fn add_texture(&mut self, texture: Arc<Texture>) {
        self.textures.push(texture);
    }

    pub fn add_character(
        &mut self,
        character: char,
        texture_index: usize,
        rect: Rect2,
        spacing: GlyphSpacing,
    ) {
        self.texture_map.insert(character, texture_index);
        self.rect_map.insert(character, rect);
        self.spacing_map.insert(character, spacing);
    }

    pub fn get_glyph_spacing(&self, current: char) -> GlyphSpacing {
        let default = GlyphSpacing::default();
        let result = self.spacing_map.get(&current).unwrap_or(&default);

        *result
    }

    pub fn get_font_spacing(&self) -> FontSpacing {
        self.spacing
    }

    pub fn get_char_source_rect(&self, current: char) -> Rect2 {
        let default = Rect2::default();
        let result = self.rect_map.get(&current).unwrap_or(&default);

        *result
    }

    pub fn get_char_dest_rect(&self, current: char) -> Rect2 {
        let default = Rect2::default();
        let result = *self.rect_map.get(&current).unwrap_or(&default);

        Rect2::new(Point2::new(0.0, 0.0), result.size)
    }

    pub fn get_texture(&self, current: char) -> Option<Arc<Texture>> {
        match self.texture_map.get(&current) {
            Some(texture_index) => { Some(
                self.textures[*texture_index].clone()
            ) },
            None => { None }
        }
    }
}
