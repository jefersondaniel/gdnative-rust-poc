use std::rc::Rc;

use gdnative::prelude::*;
use bevy::{core::CorePlugin, prelude::*};

use crate::{drawing::sprite_system::SpriteSystem, io::file_system::FileSystem, systems::menu::menu_plugin::MenuPlugin};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    app: App
}

#[methods]
impl Game {
    pub fn new(_owner: TRef<Node2D>) -> Self {
        Game {
            app: std::mem::take(
                &mut App::build()
                .insert_resource(FileSystem::new())
                .insert_resource(SpriteSystem::new())
                .add_plugin(CorePlugin::default())
                .add_plugin(MenuPlugin::default())
                .app
            )
        }
    }

    #[export]
    pub fn _process(&mut self, _owner: TRef<Node2D>, _delta: Variant) {
        self.app.update();
    }
}
