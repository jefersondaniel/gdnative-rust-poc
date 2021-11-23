use crate::io::text_section::TextSection;

pub struct NonCombatScreen {
    pub fadeintime: i32,
    pub fadeouttime: i32,
}

impl From<&TextSection> for NonCombatScreen {
    fn from(text_section: &TextSection) -> NonCombatScreen {
        NonCombatScreen {
            fadeintime: text_section.get_attribute_or_default("fadein.time"),
            fadeouttime: text_section.get_attribute_or_default("fadeout.time"),
        }
    }
}
