use std::collections::HashMap;

use bevy_ecs::prelude::*;

use crate::animations::animation_loader::AnimationLoader;
use crate::animations::animation_manager::AnimationManager;
use crate::core::configuration::Configuration;
use crate::drawing::font_map::FontMap;
use crate::drawing::mugen_font::MugenFont;
use crate::menus::menu_data::MenuData;
use crate::menus::title_screen::TitleScreen;
use crate::{core::{constants::{MUGEN_10_SYSTEM_PATH, MUGEN_11_SYSTEM_PATH}, error::DataError}, drawing::sprite_system::SpriteSystem, io::{file_system::FileSystem, text_file::TextFile}};

pub fn load_menus(
    mut commands: Commands,
    file_system: Res<FileSystem>,
    sprite_system: Res<SpriteSystem>
) -> Result<(), DataError> {
    let animation_loader = AnimationLoader::new();
    let configuration = Configuration::default();

    let textfile = load_text_file(&file_system)?;
    let menu_data = load_menu_data(&file_system, &sprite_system, &textfile)?;
    let mut sprite_file = sprite_system.get_sprite_file(&menu_data.sprite_path)?;
    let animations = animation_loader.load_animations(&menu_data.anim_path)?;
    let animation_manager = AnimationManager::new(&menu_data.anim_path, animations);

    // Screens
    let title_screen = TitleScreen::build(
        &configuration,
        &textfile,
        &mut sprite_file,
        &animation_manager
    )?;

    commands.insert_resource(menu_data);
    commands.insert_resource(title_screen);
    commands.insert_resource(configuration);

    Ok(())
}

fn load_menu_data(
    file_system: &Res<FileSystem>,
    sprite_system: &Res<SpriteSystem>,
    text_file: &TextFile
) -> Result<MenuData, DataError> {
    let info = text_file.get_section("info")?;
    let files = text_file.get_section("files")?;

    let motif_name = info.get_attribute_or_default::<String>("name");
    let motif_author = info.get_attribute_or_default::<String>("author");
    let mut font_hash_map = HashMap::<usize, MugenFont>::new();

    for i in 1..10 as usize {
        if let Some(path) = files.get_attribute::<String>(&format!("font{}", i)) {
            let font_path = format!("font/{}", &path);
            let font_path = file_system.get_path_by_refferrer(&font_path, &text_file.filepath);
            let font = sprite_system.load_font(&font_path)?;
            font_hash_map.insert(i, font);
        } else {
            break;
        }
    }

    let font_map = FontMap::new(font_hash_map);

    let sound_path = file_system.get_path_by_refferrer(
        &files.get_attribute::<String>("snd")
                .ok_or_else(|| DataError::new("Missing Files snd attribute".into()))?,
            &text_file.filepath,
    );

    let sprite_path = file_system.get_path_by_refferrer(
        &files.get_attribute::<String>("spr")
                .ok_or_else(|| DataError::new("Missing Files snd attribute".into()))?,
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
