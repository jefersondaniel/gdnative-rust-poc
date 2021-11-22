// use bevy_core::CorePlugin;
// use bevy_app::App;
use gdnative::prelude::*;

use crate::{drawing::sprite_system::SpriteSystem, io::file_system::FileSystem };

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    // app: App
}

#[methods]
impl Game {
    pub fn new(_owner: TRef<Node2D>) -> Self {
        Game { }
        // Game {
        //     app: std::mem::take(
        //         &mut App::build()
        //         .insert_resource(FileSystem::new())
        //         .insert_resource(SpriteSystem::new())
        //         .add_plugin(CorePlugin::default())
        //         .add_plugin(MenuPlugin::default())
        //         .app
        //     )
        // }
    }

    #[export]
    pub fn _process(&mut self, _owner: TRef<Node2D>, _delta: Variant) {
        // self.app.update();
        godot_print!("Hello World");
    }
}
