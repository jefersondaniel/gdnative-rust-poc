use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::ChildBuilder;

use crate::{animations::animation_manager::AnimationManager, backgrounds::background::build_background, core::{configuration::Configuration, error::DataError, regex::{RegEx, RegExFlags}}, drawing::sprite_file::SpriteFile, io::text_file::TextFile};

use super::background::Background;

#[derive(Clone)]
pub struct BackgroundGroup {
    pub backgrounds: Vec<Background>,
}

impl BackgroundGroup {
    pub fn build(
        prefix: &str,
        configuration: &Configuration,
        textfile: &TextFile,
        sprite_file: &mut SpriteFile,
        animation_manager: &AnimationManager,
    ) -> Result<Self, DataError> {
        let pattern = format!("^{}BG (.*)$", prefix);
        let regex = RegEx::new(&pattern, RegExFlags::IgnoreCase);
        let mut backgrounds = Vec::new();

        for textsection in textfile.sections.iter() {
            if regex.is_match(&textsection.title) {
                backgrounds.push(build_background(
                    configuration,
                    textsection,
                    sprite_file,
                    animation_manager
                )?);
            }
        }

        Ok(BackgroundGroup {
            backgrounds,
        })
    }

    pub fn render(&self, commands: &mut ChildBuilder, configuration: &Res<Configuration>) {
        for background in self.backgrounds.iter() {
            background.render(commands, &configuration);
        }
    }
}
