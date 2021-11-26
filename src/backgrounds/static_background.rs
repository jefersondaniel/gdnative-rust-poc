use std::{sync::Arc};

use gdnative::api::visual_server::TextureFlags;

use crate::{core::{configuration::Configuration, error::DataError, sprite_id::SpriteId}, drawing::{sprite_file::SpriteFile, texture_ref::TextureRef}, io::text_section::TextSection};

use super::base_background::BaseBackground;

pub struct StaticBackground {
    base_background: BaseBackground,
    spriteid: SpriteId,
    texture_ref: Arc<TextureRef>,
}

impl StaticBackground {
    pub fn build(
        configuration: &Configuration,
        textsection: &TextSection,
        sprite_file: &mut SpriteFile
    ) -> Result<Self, DataError> {
        let spriteid = textsection.get_attribute_or("spriteno", SpriteId::invalid());
        let sff_data = sprite_file.get_sprite(&spriteid)?;
        let texture_ref = TextureRef::allocate(sff_data, None, TextureFlags(0))?;

        Ok(StaticBackground {
            base_background: BaseBackground::build(configuration, textsection)?,
            spriteid,
            texture_ref,
        })
    }
}
