use gdnative::prelude::*;
use crate::core::game::Game;

mod animations;
mod backgrounds;
mod core;
mod drawing;
mod io;
mod menus;
mod systems;

fn init(handle: InitHandle) {
  handle.add_class::<Game>();
}

godot_init!(init);
