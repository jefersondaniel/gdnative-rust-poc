use crate::{io::{file_system, text_file::TextFile}, core::{error::DataError, constants::DATA_PATH, enumerations::PlayerSelectType}};

use super::{stage_profile::StageProfile, player_profile::{PlayerProfile, PlayerSelect}};

#[derive(Debug, Clone, Default)]
pub struct ProfileLoader {
    pub stages: Vec<StageProfile>,
    pub players: Vec<PlayerSelect>
}

fn parse_profile_line(line: &str) -> Option<(String, String)> {
    if line.len() == 0 {
        return None;
    }

    let pieces: Vec<&str> = line.split(",").collect();
    let mut player_path: String = "".to_string();
    let mut stage_path: String = "".to_string();

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
    pub fn initialize(&mut self) -> Result<(), DataError> {
        let textfile = file_system::open_text_file(
            &file_system::combine_paths(DATA_PATH, "data/select.def")
        )?;

        self.build_stage_profiles(&textfile)?;
        self.build_player_profiles(&textfile)?;

        Ok(())
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

    fn build_player_profiles(&mut self, textfile: &TextFile) -> Result<(), DataError> {
        self.players.clear();

        let textsection = textfile.get_section("Characters")?;

        for line in textsection.lines.iter() {
            let line_text = line.to_string();

            if line_text.to_lowercase() == "random" {
                self.players.push(PlayerSelect {
                    select_type: PlayerSelectType::Random,
                    profile: None
                });

                continue;
            }

            if let Some((player_path, stage_path)) = parse_profile_line(&line_text) {
                self.players.push(PlayerSelect {
                    select_type: PlayerSelectType::Profile,
                    profile: Some(PlayerProfile::build(&player_path, &stage_path)?),
                });
            }
        }

        Ok(())
    }
}
