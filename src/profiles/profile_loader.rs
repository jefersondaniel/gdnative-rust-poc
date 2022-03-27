use std::collections::HashMap;

use crate::{io::{file_system, text_file::TextFile}, core::{error::DataError, constants::DATA_PATH, enumerations::PlayerSelectType}, menus::select_screen::SelectScreen, drawing::sprite_system::SpriteSystem};

use super::{stage_profile::StageProfile, player_profile::{PlayerProfile, PlayerSelect}};

#[derive(Clone)]
pub struct ProfileLoader {
    pub stages: Vec<StageProfile>,
    pub players: Vec<Option<PlayerSelect>>,
    player_map: Option<HashMap<(i32, i32), Option<PlayerSelect>>>,
    select_screen: SelectScreen,
}

fn parse_profile_line(line: &str) -> Option<(String, String)> {
    if line.len() == 0 {
        return None;
    }

    let pieces: Vec<&str> = line.split(",").collect();
    let mut player_path: String = "".to_string();
    let mut stage_path: String = "".to_string();

    if pieces.len() == 0 {
        return None;
    }

    if pieces.len() >= 1 {
        player_path = file_system::combine_paths(
            DATA_PATH,
            &format!("chars/{}/{}.def", pieces[0].trim(), pieces[0].trim()),
        );
    }

    if pieces.len() >= 2 {
        stage_path = file_system::combine_paths(
            DATA_PATH,
            pieces[1].trim(),
        );
    }

    if !file_system::does_file_exist(&player_path) {
        return None
    }

    if stage_path.len() > 0 && !file_system::does_file_exist(&stage_path) {
        return None
    }

    Some((player_path, stage_path))
}

impl ProfileLoader {
    pub fn build(select_screen: &SelectScreen, sprite_system: &SpriteSystem) -> Result<Self, DataError> {
        let textfile = file_system::open_text_file(
            &file_system::combine_paths(DATA_PATH, "data/select.def")
        )?;

        let mut profile_loader = ProfileLoader {
            select_screen: select_screen.clone(),
            stages: Default::default(),
            players: Default::default(),
            player_map: Default::default(),
        };

        profile_loader.build_stage_profiles(&textfile)?;
        profile_loader.build_player_profiles(&textfile, sprite_system)?;
        profile_loader.select_screen = select_screen.clone();

        Ok(profile_loader)
    }

    fn build_stage_profiles(&mut self, textfile: &TextFile) -> Result<(), DataError> {
        self.stages.clear();

        let textsection = textfile.get_section("ExtraStages")?;

        for line in textsection.lines.iter() {
            let line_text = line.to_string();
            let stage_path = file_system::combine_paths(DATA_PATH, &line_text);
            let stagetextfile = file_system::open_text_file(&stage_path)?;
            let name: String = stagetextfile.get_section("Info")?.get_attribute_or_default("name");

            self.stages.push(StageProfile {
                name,
                filepath: stage_path,
            });
        }

        Ok(())
    }

    fn build_player_profiles(&mut self, textfile: &TextFile, sprite_system: &SpriteSystem) -> Result<(), DataError> {
        self.players.clear();

        let textsection = textfile.get_section("Characters")?;

        for line in textsection.lines.iter() {
            let line_text = line.to_string();

            if line_text.to_lowercase() == "random" {
                self.players.push(Some(PlayerSelect {
                    select_type: PlayerSelectType::Random,
                    profile: None
                }));

                continue;
            }

            if let Some((player_path, stage_path)) = parse_profile_line(&line_text) {
                self.players.push(Some(PlayerSelect {
                    select_type: PlayerSelectType::Profile,
                    profile: Some(PlayerProfile::build(&player_path, &stage_path, &sprite_system)?),
                }));

                continue;
            }

            self.players.push(None);
        }

        Ok(())
    }

    pub fn get_player_on_grid(&mut self, position: (i32, i32)) -> Option<PlayerSelect> {
        let player_map = &self.player_map;
        let result = match player_map {
            Some(map) => map.get(&position).cloned(),
            None => {
                let mut map = HashMap::<(i32, i32), Option<PlayerSelect>>::new();
                let mut index = 0;

                for y in 0..self.select_screen.rows {
                    for x in 0..self.select_screen.columns {
                        if index >= self.players.len() {
                            break;
                        }

                        map.insert((x, y), self.players[index].clone());
                        index += 1;
                    }

                    if index >= self.players.len() {
                        break;
                    }
                }

                self.player_map = Some(map.clone());
                map.get(&position).cloned()
            }
        };

        match result {
            Some(select) => select,
            None => None,
        }
    }
}
