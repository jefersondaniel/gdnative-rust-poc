use std::sync::{Arc, Mutex};
use gdnative::{api::{Node2D, AudioStreamPlayer, AudioStream}, Ref, object::AsArg, prelude::Unique, godot_error};

use crate::core::error::DataError;

pub struct Audio {
    node: Arc<Mutex<Ref<Node2D, Unique>>>,
    player_pool: Vec<Arc<Mutex<Ref<AudioStreamPlayer, Unique>>>>,
    player_pool_cursor: usize,
}

impl Audio {
    pub fn new(node: Arc<Mutex<Ref<Node2D, Unique>>>) -> Self {
        Audio {
            node,
            player_pool: Vec::new(),
            player_pool_cursor: 0,
        }
    }

    pub fn play(&mut self, stream: impl AsArg<AudioStream>) {
        let player_result = self.get_player();

        match player_result {
            Ok(player_lock) => {
                match player_lock.lock() {
                    Ok(player) => {
                        player.set_stream(stream);
                        player.play(0.0);
                    },
                    Err(_) => {
                        godot_error!("Could not lock player mutext");
                    }
                }
            },
            Err(error) => {
                godot_error!("{}", error);
            }
        }
    }

    fn create_player(&self) -> Result<Ref<AudioStreamPlayer>, DataError> {
        let node = self.node.lock().map_err(|_| DataError::new("Can't get root node mutext".to_string()))?;
        let player = AudioStreamPlayer::new();
        let player_shared = player.into_shared();
        node.add_child(player_shared, false);
        return Ok(player_shared);
    }

    fn get_player(&mut self) -> Result<Arc<Mutex<Ref<AudioStreamPlayer, Unique>>>, DataError> {
        let mut result: Option<Arc<Mutex<Ref<AudioStreamPlayer, Unique>>>> = None;

        for _ in 0..self.player_pool.len() {
            if let Ok(player) = self.player_pool[self.player_pool_cursor].try_lock() {
                if !player.is_playing() {
                    result = Some(self.player_pool[self.player_pool_cursor].clone());
                    break;
                }
            }

            self.player_pool_cursor += 1;

            if self.player_pool_cursor >= self.player_pool.len() {
                self.player_pool_cursor -= self.player_pool.len();
            }
        }

        if let Some(result) = result {
            return Ok(result);
        }

        let player = self.create_player()?;
        let player_ref = Arc::new(Mutex::new(unsafe { player.assume_unique() }));
        self.player_pool.push(player_ref.clone());

        Ok(player_ref)
    }
}
