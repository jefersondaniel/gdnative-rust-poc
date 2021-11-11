use gdnative::prelude::*;
use crate::core::game::Game;

mod core;
mod io;

fn init(handle: InitHandle) {
  handle.add_class::<Game>();
}

godot_init!(init);
