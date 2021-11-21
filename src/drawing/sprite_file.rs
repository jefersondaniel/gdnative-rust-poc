use std::collections::HashMap;

use crate::core::{error::DataError, sprite_id::SpriteId};

use super::sff::{sff_common::{SffData, SffMetadata}, sff_parser};

pub struct SpriteFile {
    pub path: String,
    pub metadata: SffMetadata,
    cache: HashMap<SpriteId, SffData>,
}

impl SpriteFile {
    pub fn load(path: &str) -> Result<Self, DataError> {
        let metadata = sff_parser::read_metadata(&path)?;

        Ok(SpriteFile {
            path: path.to_string(),
            cache: HashMap::new(),
            metadata,
        })
    }

    pub fn has_palettes(&self) -> bool { self.metadata.verhi == 2 }

    pub fn get_sprite(&mut self, sprite_id: &SpriteId) -> Result<SffData, DataError> {
        if !self.cache.contains_key(sprite_id) {
            let images = sff_parser::read_images(&self.path, &[sprite_id.group])?;

            for image in images.iter() {
                let new_key = SpriteId::new(image.groupno, image.imageno);
                self.cache.insert(new_key, image.clone());
            }
        }

        let sff_data = self.cache.get(sprite_id)
            .ok_or_else(|| DataError::new(format!("Image not found: {}", sprite_id)))?;

        Ok(sff_data.clone())
    }

    pub fn load_all_sprites(&mut self) -> Result<(), DataError> {
        let images = sff_parser::read_images(&self.path, &[])?;

        for image in images.iter() {
            let new_key = SpriteId::new(image.groupno, image.imageno);
            self.cache.insert(new_key, image.clone());
        }

        Ok(())
    }
}
