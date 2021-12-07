use std::{collections::HashMap, sync::Arc};

use gdnative::core_types::Rect2;

use crate::systems::visual_server::texture::Texture;

use super::common::{GlyphSpacing, FontSpacing};

pub struct BitmapFont {
    texture_map: HashMap<char, Arc<Texture>>,
    rect_map: HashMap<char, Rect2>,
    spacing_map: HashMap<char, GlyphSpacing>,
    spacing: FontSpacing
}

impl BitmapFont {
    pub fn new(spacing: FontSpacing) -> Self {
        Self {
            texture_map: HashMap::new(),
            rect_map: HashMap::new(),
            spacing_map: HashMap::new(),
            spacing
        }
    }

    pub fn add_character(
        &mut self,
        character: char,
        texture: Arc<Texture>,
        rect: Rect2,
        spacing: GlyphSpacing,
    ) {
        self.texture_map.insert(character, texture);
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

    pub fn get_char_rect(&self, current: char) -> Rect2 {
        let default = Rect2::default();
        let result = self.rect_map.get(&current).unwrap_or(&default);

        *result
    }

    pub fn get_texture(&self, current: char) -> Option<Arc<Texture>> {
        match self.texture_map.get(&current) {
            Some(texture) => { Some(texture.clone()) },
            None => { None }
        }
    }
}
