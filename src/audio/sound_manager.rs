use std::collections::HashMap;

use crate::core::{sound_id::SoundId, error::DataError};

use super::{structs::WavSound, snd_parser::read_sounds};

#[derive(Clone)]
pub struct SoundManager {
    sound_map: HashMap<SoundId, WavSound>,
}

impl SoundManager {
    pub fn load(path: &str) -> Result<Self, DataError> {
        let sounds = read_sounds(path)?;
        let mut sound_map = HashMap::new();

        for sound in sounds.iter() {
            sound_map.insert(sound.soundid, sound.clone());
        }

        Ok(SoundManager { sound_map })
    }

    pub fn get_sound(&self, soundid: SoundId) -> Option<&WavSound> {
        return self.sound_map.get(&soundid)
    }
}
