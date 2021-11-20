use crate::drawing::font_map::FontMap;

pub struct MenuData {
    pub motif_name: String,
    pub motif_author: String,
    pub font_map: FontMap,
    pub sound_path: String,
    pub sprite_path: String,
    pub anim_path: String,
}

impl MenuData {
    pub fn new(
        motif_name: String,
        motif_author: String,
        font_map: FontMap,
        sound_path: String,
        sprite_path: String,
        anim_path: String,
    ) -> Self {
        MenuData {
            motif_name,
            motif_author,
            font_map,
            sound_path,
            sprite_path,
            anim_path
        }
    }
}
