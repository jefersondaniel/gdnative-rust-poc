use gdnative::core_types::Point2;

use crate::{elements::element::Element, io::text_file::TextFile, core::{error::DataError, configuration::Configuration}, drawing::sprite_file::SpriteFile, animations::animation_manager::AnimationManager};

use super::non_combat_screen::NonCombatScreen;

#[derive(Clone)]
pub struct SelectScreen {
    pub non_combat_screen: NonCombatScreen,
    pub cellbg: Element,
    pub cellrandom: Element,
    pub columns: i32,
    pub rows: i32,
    pub wrapping: bool,
    pub showemptyboxes: bool,
    pub moveoveremptyboxes: bool,
    pub grid_position: Point2,
    pub cellsize: Point2,
    pub cellspacing: i32,
}

impl SelectScreen {
    pub fn build(
        configuration: &Configuration,
        textfile: &TextFile,
        sprite_file: &mut SpriteFile,
        animation_manager: &AnimationManager,
    ) -> Result<SelectScreen, DataError> {
        let textsection = textfile.get_section("Select Info")?;

        let non_combat_screen = NonCombatScreen::build(
            "Select",
            configuration,
            &textsection,
            textfile,
            sprite_file,
            animation_manager
        )?;

        let cellbg = Element::build(&textsection, "cell.bg", sprite_file)?;
        let cellrandom = Element::build(&textsection, "cell.random", sprite_file)?;
        let columns = textsection.get_attribute_or_default("columns");
        let rows = textsection.get_attribute_or_default("rows");
        let wrapping = textsection.get_attribute_or_default("wrapping");
        let showemptyboxes = textsection.get_attribute_or_default("showemptyboxes");
        let moveoveremptyboxes = textsection.get_attribute_or_default("moveoveremptyboxes");
        let grid_position = textsection.get_attribute_or_default("pos");
        let cellsize = textsection.get_attribute_or_default("cell.size");
        let cellspacing = textsection.get_attribute_or_default("cell.spacing");

        Ok(SelectScreen {
            non_combat_screen,
            cellbg,
            cellrandom,
            columns,
            rows,
            wrapping,
            showemptyboxes,
            moveoveremptyboxes,
            grid_position,
            cellsize,
            cellspacing,
        })
    }
}
