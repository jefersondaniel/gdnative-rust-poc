use crate::{animations::animation_manager::AnimationManager, backgrounds::{background::Background, background_group::BackgroundGroup}, core::{configuration::Configuration, error::DataError, regex::RegEx, regex::RegExFlags}, drawing::sprite_file::SpriteFile, io::{text_file::TextFile, text_section::TextSection}};

pub struct NonCombatScreen {
    pub fadeintime: i32,
    pub fadeouttime: i32,
    pub background_group: BackgroundGroup,
}

impl NonCombatScreen {
    pub fn build(
        prefix: &str,
        configuration: &Configuration,
        textsection: &TextSection,
        textfile: &TextFile,
        sprite_file: &mut SpriteFile,
        animation_manager: &AnimationManager,
    ) -> Result<NonCombatScreen, DataError> {
        Ok(NonCombatScreen {
            fadeintime: textsection.get_attribute_or_default("fadein.time"),
            fadeouttime: textsection.get_attribute_or_default("fadeout.time"),
            background_group: BackgroundGroup::build(
                prefix,
                configuration,
                textfile,
                sprite_file,
                animation_manager
            )?
        })
    }
}
