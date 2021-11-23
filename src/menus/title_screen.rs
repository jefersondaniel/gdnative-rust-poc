use gdnative::core_types::Vector2;

use crate::{core::{attribute_value::AttributeValue, error::DataError, sound_id::SoundId}, drawing::print_data::PrintData, io::text_section::TextSection};

use super::non_combat_screen::NonCombatScreen;

pub struct TitleScreen {
    non_combat_screen: NonCombatScreen,
    menuposition: Vector2,
    mainfont: PrintData,
    activefont: PrintData,
    spacing: Vector2,
    visiblemenuitems: i32,
    cursorvisible: bool,
    soundcursormove: Option<SoundId>,
    soundselect: Option<SoundId>,
    soundcancel: Option<SoundId>,
}

impl TitleScreen {
    pub fn build(textsection: &TextSection) -> Result<TitleScreen, DataError> {
        Ok(TitleScreen {
            non_combat_screen: NonCombatScreen::from(textsection),
			menuposition: textsection.get_attribute_or_fail("menu.pos")?,
			mainfont: textsection.get_attribute_or_fail("menu.item.font")?,
			activefont: textsection.get_attribute_or_fail("menu.item.active.font")?,
			spacing: textsection.get_attribute_or_fail("menu.item.spacing")?,
			visiblemenuitems: textsection.get_attribute_or_fail("menu.window.visibleitems")?,
			cursorvisible: textsection.get_attribute_or_fail("menu.boxcursor.visible")?,
			soundcursormove: textsection.get_attribute("cursor.move.snd"),
			soundselect: textsection.get_attribute("cursor.done.snd"),
			soundcancel: textsection.get_attribute("cancel.snd"),
        })
    }
}
