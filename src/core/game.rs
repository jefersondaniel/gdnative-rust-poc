use gdnative::prelude::*;
use crate::io::file_system::FileSystem;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {}

#[methods]
impl Game {
    pub fn new(_owner: &Node2D) -> Self {
        Game {}
    }

    #[export]
    pub fn _ready(&self, _owner: &Node2D) {
        godot_print!("Game Started");

        let filesystem = FileSystem::new();
        let text_file_result = filesystem.open_text_file("res://data/data/mugen1/system.def".to_string());

        match text_file_result {
            Ok(text_file) => {
                for section in text_file.sections.iter() {
                    godot_print!("Section: '{}'", section.title);
                    for (key, value) in section.parsedlines.iter() {
                        godot_print!("'{}' = '{}'", key, String::from(value));
                    }
                }
                godot_print!("Read Success");
            },
            Err(error) => {
                godot_print!("{}", error);
            }
        }
    }
}
