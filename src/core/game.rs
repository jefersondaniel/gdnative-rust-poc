use gdnative::prelude::*;
use bevy::{core::CorePlugin, prelude::*};

use crate::{animations::animation_loader::AnimationLoader, io::file_system::FileSystem};
// use crate::io::file_system::FileSystem;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    app: App
}

fn startup() {
    let fs = FileSystem::new();
    let animation_loader = AnimationLoader::new();
    let text_file_result = fs.open_text_file("res://data/chars/kfm/kfm.air");

    if let Ok(text_file) = text_file_result {
        godot_print!("Opened");
        let animations = animation_loader.load_animations(text_file);
        let animation = animations.get(&5300).unwrap();
        for element in animation.elements.iter() {
            godot_print!("{}", element);
        }
        godot_print!("animation total time = {}", animation.totaltime);
    }
}

fn print_all() {
    // godot_print!("print all");
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
