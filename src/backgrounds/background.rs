use crate::{animations::animation_manager::AnimationManager, core::{error::DataError, sprite_id::SpriteId}, drawing::{sprite_file::SpriteFile, texture_ref::TextureRef}, io::text_section::TextSection};

use super::background_type::BackgroundType;

pub struct StaticBackground {
    spriteid: SpriteId,
    sprite: TextureRef,
}

pub enum Background {
    None
}

pub fn build_background(
    section: &TextSection,
    sprite_file: &SpriteFile,
    animation_manager: &AnimationManager
) -> Result<Background, DataError> {
    let background_type: BackgroundType = section.get_attribute_or_default("type");

    match background_type {
        BackgroundType::Static => build_static_background(section, sprite_file),
        BackgroundType::Parallax => build_parallax_background(section, sprite_file),
        BackgroundType::Animated => build_animated_background(section, sprite_file, animation_manager),
        BackgroundType::None => Ok(Background::None),
    }
}

fn build_static_background(
    section: &TextSection,
    sprite_file: &SpriteFile
) -> Result<Background, DataError> {
    Ok(Background::None)
}

fn build_parallax_background(
    section: &TextSection,
    sprite_file: &SpriteFile
) -> Result<Background, DataError> {
    Ok(Background::None)
}

fn build_animated_background(
    section: &TextSection,
    sprite_file: &SpriteFile,
    animation_manager: &AnimationManager
) -> Result<Background, DataError> {
    Ok(Background::None)
}
