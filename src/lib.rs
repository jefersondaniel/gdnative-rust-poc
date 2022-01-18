use gdnative::prelude::*;
use crate::core::game::Game;

mod animations;
mod audio;
mod backgrounds;
mod core;
mod drawing;
mod elements;
mod io;
mod menus;
mod systems;

fn init(handle: InitHandle) {
  handle.add_class::<Game>();
}

godot_init!(init);
