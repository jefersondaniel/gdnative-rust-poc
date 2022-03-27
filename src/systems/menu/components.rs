use std::sync::{Arc, RwLock};

use crate::{audio::sound_manager::SoundManager, drawing::sprite_file::SpriteFile};

pub struct MenuSoundManager(pub SoundManager);

pub struct MenuSpriteFile(pub Arc<RwLock<SpriteFile>>);
