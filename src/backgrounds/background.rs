use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::ChildBuilder;

use crate::{animations::animation_manager::AnimationManager, core::{configuration::Configuration, error::DataError}, drawing::{sprite_file::SpriteFile}, io::text_section::TextSection};

use super::{background_type::BackgroundType, static_background::StaticBackground};

#[derive(Clone)]
pub enum Background {
    None,
    Static(StaticBackground),
}

struct Empty;

pub fn build_background(
    configuration: &Configuration,
    textsection: &TextSection,
    sprite_file: &mut SpriteFile,
    animation_manager: &AnimationManager
) -> Result<Background, DataError> {
    let background_type: BackgroundType = textsection.get_attribute_or_default("type");

    match background_type {
        BackgroundType::Static => build_static_background(configuration, textsection, sprite_file),
        BackgroundType::Parallax => build_parallax_background(textsection, sprite_file),
        BackgroundType::Animated => build_animated_background(textsection, sprite_file, animation_manager),
        BackgroundType::None => Ok(Background::None),
    }
}

fn build_static_background(
    configuration: &Configuration,
    textsection: &TextSection,
    sprite_file: &mut SpriteFile
) -> Result<Background, DataError> {
    Ok(Background::Static(StaticBackground::build(
        configuration,
        textsection,
        sprite_file
    )?))
}

fn build_parallax_background(
    textsection: &TextSection,
    sprite_file: &SpriteFile
) -> Result<Background, DataError> {
    Ok(Background::None)
}

fn build_animated_background(
    textsection: &TextSection,
    sprite_file: &SpriteFile,
    animation_manager: &AnimationManager
) -> Result<Background, DataError> {
    Ok(Background::None)
}

impl Background {
    pub fn render(&self, commands: &mut ChildBuilder, configuration: &Res<Configuration>) -> Entity {
        match self {
            Background::Static(static_background) => {
                static_background.render(commands, &configuration)
            },
            _ => {
                commands.spawn().insert(Empty).id()
            }
        }
    }
}
