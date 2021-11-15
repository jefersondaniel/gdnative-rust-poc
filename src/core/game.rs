use gdnative::prelude::*;
use bevy::{core::CorePlugin, prelude::*};
// use crate::io::file_system::FileSystem;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    app: App
}

fn startup() {
    godot_print!("startup");
}

fn print_all() {
    godot_print!("print all");
}

#[methods]
impl Game {
    pub fn new(_owner: TRef<Node2D>) -> Self {
        Game {
            app: std::mem::take(
                &mut App::build()
                .add_plugin(CorePlugin::default())
                .add_startup_system(startup.system())
                .add_system(print_all.system())
                .app
            )
        }
    }

    #[export]
    pub fn _ready(&mut self, _owner: TRef<Node2D>) {
        self.app.update();
    }

    #[export]
    pub fn _process(&mut self, _owner: TRef<Node2D>, _delta: Variant) {
        self.app.update();
    }
}
