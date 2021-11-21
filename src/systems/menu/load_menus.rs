use std::collections::HashMap;

use bevy::prelude::*;
use gdnative::prelude::*;

use crate::drawing::font::Font;
use crate::drawing::font_map::FontMap;
use crate::menus::menu_data::MenuData;
use crate::{core::{constants::{MUGEN_10_SYSTEM_PATH, MUGEN_11_SYSTEM_PATH}, error::DataError}, drawing::sprite_system::SpriteSystem, io::{file_system::FileSystem, text_file::TextFile}};

pub fn load_menus(
    file_system: Res<FileSystem>,
    sprite_system: Res<SpriteSystem>
) {
    match load_text_file(&file_system) {
        Ok(text_file) => {
            match load_menu_data(&file_system, &sprite_system, &text_file) {
                Ok(menu_data) => {
                    godot_print!("Menu data found");
                    godot_print!("{}", menu_data.motif_name);
                    godot_print!("{}", menu_data.motif_author);
                    godot_print!("{}", menu_data.sound_path);
                    godot_print!("{}", menu_data.sprite_path);
                    godot_print!("{}", menu_data.anim_path);
                },
                Err(error) => {
                    godot_error!("{}", error);
                }
            }
        }
        Err(error) => {
            godot_error!("{}", error);
        }
    }
}

fn load_menu_data(
    file_system: &Res<FileSystem>,
    sprite_system: &Res<SpriteSystem>,
    text_file: &TextFile
) -> Result<MenuData, DataError> {
    let info = text_file.get_section("info")
        .ok_or_else(|| DataError::new("Missing Info section".into()))?;
    let files = text_file.get_section("files")
        .ok_or_else(|| DataError::new("Missing Files section".into()))?;

    let motif_name = info.get_attribute("name").unwrap_or_default().to_string();
    let motif_author = info.get_attribute("author").unwrap_or_default().to_string();
    let mut font_hash_map = HashMap::<usize, Font>::new();

    for i in 1..10 as usize {
        if let Some(path) = files.get_attribute(&format!("font{}", i)) {
            let font = sprite_system.load_font(&path.to_string())?;
            font_hash_map.insert(i, font);
        } else {
            break;
        }
    }

    let font_map = FontMap::new(font_hash_map);

    let sound_path = file_system.get_path_by_refferrer(
        &files.get_attribute("snd")
                .ok_or_else(|| DataError::new("Missing Files snd attribute".into()))?
                .to_string(),
            &text_file.filepath,
    );

    let sprite_path = file_system.get_path_by_refferrer(
        &files.get_attribute("spr")
                .ok_or_else(|| DataError::new("Missing Files snd attribute".into()))?
                .to_string(),
            &text_file.filepath,
    );

    let anim_path = text_file.filepath.clone();

    Ok(MenuData::new(
        motif_name,
        motif_author,
        font_map,
        sound_path,
        sprite_path,
        anim_path
    ))
}

fn load_text_file(file_system: &Res<FileSystem>) -> Result<TextFile, DataError> {
    if file_system.does_file_exist(MUGEN_11_SYSTEM_PATH) {
        return file_system.open_text_file(MUGEN_11_SYSTEM_PATH);
    }

    file_system.open_text_file(MUGEN_10_SYSTEM_PATH)
}
