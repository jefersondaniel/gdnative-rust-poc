use crate::io::text_section::TextSection;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game;

#[methods]
impl Game {
  pub fn new(_owner: &Node2D) -> Self {
    Game {}
  }

  #[export]
  pub fn _ready(&self, _owner: &Node2D) {
    godot_print!("Game Started")
  }
}
