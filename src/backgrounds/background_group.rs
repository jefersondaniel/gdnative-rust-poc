use crate::{animations::animation_manager::AnimationManager, backgrounds::background::build_background, core::{error::DataError, regex::{RegEx, RegExFlags}}, drawing::sprite_file::SpriteFile, io::text_file::TextFile};

use super::background::Background;

pub struct BackgroundGroup {
    pub backgrounds: Vec<Background>,
}

impl BackgroundGroup {
    pub fn build(
        prefix: &str,
        textfile: &TextFile,
        sprite_file: &SpriteFile,
        animation_manager: &AnimationManager,
    ) -> Result<Self, DataError> {
        let pattern = format!("^{}BG (.*)$", prefix);
        let regex = RegEx::new(&pattern, RegExFlags::IgnoreCase);
        let mut backgrounds = Vec::new();

        for section in textfile.sections.iter() {
            if regex.is_match(&section.title) {
                backgrounds.push(build_background(
                    section,
                    sprite_file,
                    animation_manager
                )?);
            }
        }

        Ok(BackgroundGroup {
            backgrounds,
        })
    }
}
