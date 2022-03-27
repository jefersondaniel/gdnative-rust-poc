use bevy_ecs::{prelude::{Commands}, system::EntityCommands};
use gdnative::{core_types::{Point2, Vector2, Transform2D}, api::visual_server::TextureFlags};

use crate::{core::{sprite_id::SpriteId, sound_id::SoundId, enumerations::{SpriteEffects, ElementType}, error::DataError}, io::text_section::TextSection, drawing::{print_data::PrintData, sprite_file::SpriteFile}, systems::visual_server::sprite::{SpriteBundle, Sprite}};

#[derive(Clone)]
pub struct Element {
    pub prefix: String,
    pub animationnumber: i32,
    pub spriteid: SpriteId,
    pub fontdata: PrintData,
    pub text: String,
    pub soundid: SoundId,
    pub sndtime: i32,
    pub offset: Point2,
    pub displaytime: i32,
    pub flip: SpriteEffects,
    pub layerno: i32,
    pub scale: Vector2,
    pub element_type: ElementType,
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
        })
    }

    pub fn render<'a, 'b>(
        &self,
        commands: &'b mut Commands<'a>,
        sprite_file: &mut SpriteFile,
        position: Point2
    ) -> EntityCommands<'a, 'b> {
        let sff = sprite_file.get_sprite(&self.spriteid);

        if let Err(error) = sff {
            gdnative::godot_error!("Can't render element with sprite id: {}", self.spriteid);

            return commands.spawn();
        }

        let sff = sff.unwrap();
        let texture = sff.create_texture(None, TextureFlags(0)).unwrap();
        let sff_offset = sff.offset();

        commands.spawn_bundle(SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                size: texture.size,
                offset: Point2::new(
                    sff_offset.x + self.offset.x,
                    sff_offset.y + self.offset.y
                ),
                ..Default::default()
            },
            transform: Transform2D::translation(position.x, position.y),
            ..Default::default()
        })
    }
}
