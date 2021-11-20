use gdnative::prelude::*;
use crate::core::game::Game;

mod core;
mod io;
mod drawing;
mod animations;
mod systems;
mod menus;

fn init(handle: InitHandle) {
  handle.add_class::<Game>();
}

godot_init!(init);
