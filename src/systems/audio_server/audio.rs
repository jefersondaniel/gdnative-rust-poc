use std::sync::{Arc, Mutex};
use gdnative::{api::{Node2D, AudioStreamPlayer, AudioStream}, Ref, object::AsArg};

pub struct Audio {
    node: Arc<Mutex<Ref<Node2D>>>,
}

impl Audio {
    pub fn new(node: Arc<Mutex<Ref<Node2D>>>) -> Self {
        Audio {
            node,
        }
    }

    pub fn get_player(&self) -> Ref<AudioStreamPlayer> {
        let node_guard = self.node.lock().expect("Failed to get root node lock");
        let node = unsafe { node_guard.assume_safe() };
        let player = AudioStreamPlayer::new();
        let player_shared = player.into_shared();
        node.add_child(player_shared, false);
        return player_shared;
    }

    pub fn play(&self, stream: impl AsArg<AudioStream>) {
        let player = unsafe { self.get_player().assume_safe() };
        player.set_stream(stream);
        player.play(0.0);
    }
}
