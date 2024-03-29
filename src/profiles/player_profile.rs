use std::{collections::{BTreeMap}, sync::{Arc, RwLock}};

use gdnative::core_types::Size2;

use crate::{core::{error::DataError, enumerations::PlayerSelectType}, io::{file_system, text_section::TextSection}, drawing::{sprite_system::SpriteSystem, sprite_file::SpriteFile}};

#[derive(Clone)]
pub struct PlayerProfile {
    pub definition_path: String,
    pub player_name: String,
    pub display_name: String,
    pub author: String,
    pub version: String,
    pub localcoord: Size2,
    pub mugen_version: String,
    pub palette_order: Vec<usize>,
    pub command_path: String,
    pub constants_path: String,
    pub state_files: Vec<String>,
    pub common_state_file: String,
    pub sprite_path: String,
    pub animation_path: String,
    pub sound_path: String,
    pub stage_path: String,
    pub palette_files: BTreeMap<usize, String>,
    pub base_path: String,
    pub sprite_file: Arc<RwLock<SpriteFile>>,
}

#[derive(Clone)]
pub struct PlayerSelect {
    pub select_type: PlayerSelectType,
    pub profile: Option<PlayerProfile>,
}

fn build_palette_order(input: String) -> Vec<usize> {
    let text = input.trim();
    let mut result: Vec<usize> = Vec::new();
    for raw_item in text.split(",") {
        let item = raw_item.trim().to_string();
        if item.len() == 0 {
            continue;
        }
        let value = item.parse::<usize>();
        if let Ok(value) = value {
            result.push(value);
        }
    }
    result
}

fn combine_paths(base_path: &str, path: String) -> String {
    if path.len() > 0 {
        file_system::combine_paths(base_path, &path)
    } else {
        String::new()
    }
}

fn get_common_state_file(base_path: &str, input: String) -> String {
    if input.to_lowercase() == "common1.cns" {
        let try_path = combine_paths(base_path, input);

        if file_system::does_file_exist(&try_path) {
            return try_path;
        }

        return "res://data/data/common1.cns".to_string();
    }

    return combine_paths(base_path, input);
}

fn build_state_files(
    section: &TextSection,
    base_path: &str,
    common_state_file: &str,
    command_path: &str,
) -> Vec<String> {
    let mut files: BTreeMap::<i32, String> = BTreeMap::new();
    files.insert(-2, common_state_file.to_string());
    files.insert(-1, command_path.to_string());

    for (key, value) in section.parsedlines.iter() {
        let attribute_text = value.to_string();

        if !value.compare("st", 0, 0, 2) {
            continue;
        }

        if attribute_text.to_lowercase() == "st" {
            let path = combine_paths(base_path, attribute_text);
            if path.len() > 0 {
                files.insert(0, path);
            }
        } else {
            let counter = key[2..].parse::<i32>();
            if let Ok(counter) = counter {
                let path = combine_paths(base_path, attribute_text);
                if path.len() > 0 {
                    files.insert(counter, path);
                }
            }
        }
    }

    let mut result = Vec::<String>::new();

    for value in files.values() {
        result.push(value.clone());
    }

    result
}

pub fn build_palette_files(section: &TextSection, base_path: &str) -> BTreeMap<usize, String> {
    let mut palettes = BTreeMap::new();

    for (key, value) in section.parsedlines.iter() {
        let attribute_text = value.to_string();

        if !value.compare("pal", 0, 0, 3) {
            continue;
        }

        let counter = key[3..].parse::<usize>();

        if let Ok(counter) = counter {
            let path = combine_paths(base_path, attribute_text);
            palettes.insert(counter, path);
        }
    }

    return palettes;
}

impl PlayerProfile {
    pub fn build(
        definition_path: &str,
        stage_path: &str,
        sprite_system: &SpriteSystem,
    ) -> Result<PlayerProfile, DataError> {
        let textfile = file_system::open_text_file(definition_path)?;
        let infosection = textfile.get_section("info")?;
        let filesection = textfile.get_section("files")?;
        let base_path = file_system::get_directory(&textfile.filepath);
        let common_state_file = get_common_state_file(&base_path, filesection.get_attribute_or_default("stcommon"));
        let command_path = combine_paths(&base_path, filesection.get_attribute_or_default("cmd"));
        let sprite_path = combine_paths(&base_path, filesection.get_attribute_or_default("sprite"));
        let sprite_file = Arc::new(RwLock::new(sprite_system.get_sprite_file(&sprite_path)?));

        Ok(PlayerProfile {
            player_name: infosection.get_attribute_or_default("name"),
            display_name: infosection.get_attribute_or_default("displayname"),
            author: infosection.get_attribute_or_default("author"),
            version: infosection.get_attribute_or_default("versiondate"),
            mugen_version: infosection.get_attribute_or_default("mugenversion"),
            localcoord: infosection.get_attribute_or_default("localcoord"),
            palette_order: build_palette_order(infosection.get_attribute_or_default("pal.defaults")),
            constants_path: combine_paths(&base_path, filesection.get_attribute_or_default("cns")),
            state_files: build_state_files(&filesection, &base_path, &common_state_file, &command_path),
            sprite_path,
            animation_path: combine_paths(&base_path, filesection.get_attribute_or_default("anim")),
            sound_path: combine_paths(&base_path, filesection.get_attribute_or_default("sound")),
            stage_path: stage_path.to_string(),
            palette_files: build_palette_files(&filesection, &base_path),
            definition_path: definition_path.to_string(),
            base_path,
            command_path,
            common_state_file,
            sprite_file,
        })
    }
}
