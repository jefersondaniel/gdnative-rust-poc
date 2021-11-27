use std::{sync::Arc};

use gdnative::{api::{visual_server::TextureFlags}, core_types::{Point2}};

use crate::{core::{configuration::Configuration, error::DataError, sprite_id::SpriteId}, drawing::{sprite_file::SpriteFile}, io::text_section::TextSection, systems::visual_server::{sprite::Sprite, texture::Texture}};

use super::base_background::BaseBackground;

pub struct StaticBackground {
    base_background: BaseBackground,
    spriteid: SpriteId,
    texture: Arc<Texture>,
    sprite: Sprite,
}

impl StaticBackground {
    pub fn build(
        configuration: &Configuration,
        textsection: &TextSection,
        sprite_file: &mut SpriteFile
    ) -> Result<Self, DataError> {
        let spriteid = textsection.get_attribute_or("spriteno", SpriteId::invalid());
        let sff_data = sprite_file.get_sprite(&spriteid)?;
        let texture = sff_data.create_texture(None, TextureFlags(0))?;
        let sprite = Sprite {
            offset: Point2::new(sff_data.x as f32, sff_data.y as f32),
            size: texture.size,
            ..Default::default()
        };

        Ok(StaticBackground {
            base_background: BaseBackground::build(configuration, textsection)?,
            spriteid,
            texture,
            sprite,
        })
    }
}
