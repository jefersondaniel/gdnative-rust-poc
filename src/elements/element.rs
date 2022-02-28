use std::sync::Arc;

use gdnative::{core_types::{Point2, Vector2}, api::visual_server::TextureFlags};

use crate::{core::{sprite_id::SpriteId, sound_id::SoundId, enumerations::{SpriteEffects, ElementType}, error::DataError}, io::text_section::TextSection, drawing::{print_data::PrintData, sprite_file::SpriteFile}, systems::visual_server::texture::Texture};

pub struct Element {
    prefix: String,
    animationnumber: i32,
    spriteid: SpriteId,
    fontdata: PrintData,
    text: String,
    soundid: SoundId,
    sndtime: i32,
    offset: Point2,
    displaytime: i32,
    flip: SpriteEffects,
    layerno: i32,
    scale: Vector2,
    element_type: ElementType,
    texture: Arc<Texture>,
}

impl Element {
    pub fn build(
        textsection: &TextSection,
        prefix: &str,
        sprite_file: &mut SpriteFile
    ) -> Result<Element, DataError> {
        let mut flip = SpriteEffects::None;

        if textsection.get_attribute_or_default::<i32>(&format!("{}.facing", prefix)) > 0 {
            flip |= SpriteEffects::FlipHorizontally;
        }

        if textsection.get_attribute_or_default::<i32>(&format!("{}.vfacing", prefix)) > 0 {
            flip |= SpriteEffects::FlipVertically;
        }

        let animationnumber = textsection.get_attribute_or(&format!("{}.anim", prefix), -1);
        let spriteid = textsection.get_attribute_or(&format!("{}.spr", prefix), SpriteId::invalid());
        let fontdata = textsection.get_attribute_or_default(&format!("{}.font", prefix));
        let text = textsection.get_attribute_or_default(&format!("{}.text", prefix));
        let mut element_type = ElementType::None;

        if animationnumber > 0 {
            element_type = ElementType::Animation;
        } else if spriteid != SpriteId::invalid() {
            element_type = ElementType::Static;
        } else if fontdata != PrintData::default() {
            element_type = ElementType::Text;
        }

        let sff_data = sprite_file.get_sprite(&spriteid)?;
        let texture = sff_data.create_texture(None, TextureFlags(0))?;

        Ok(Element {
            flip,
            element_type,
            animationnumber,
            spriteid,
            fontdata,
            text,
            prefix: prefix.to_string(),
            soundid: textsection.get_attribute_or(&format!("{}.snd", prefix), SoundId::invalid()),
            sndtime: textsection.get_attribute_or_default(&format!("{}.sndtime", prefix)),
            offset: textsection.get_attribute_or_default(&format!("{}.offset", prefix)),
            displaytime: textsection.get_attribute_or_default(&format!("{}.displaytime", prefix)),
            layerno: textsection.get_attribute_or_default(&format!("{}.layerno", prefix)),
            scale: textsection.get_attribute_or(&format!("{}.scale", prefix), Vector2::new(1.0, 1.0)),
            texture,
        })
    }
}
