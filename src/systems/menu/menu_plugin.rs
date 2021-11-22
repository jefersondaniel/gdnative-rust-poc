use bevy_app::{AppBuilder, Plugin};
use bevy_ecs::prelude::*;

use super::load_menus::load_menus;

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_menus.system());
    }
}
