use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};

use crate::{systems::log::handle_error, menus::menu_state::MenuState};

use super::{load_menus::load_menus, show_screen::{show_title_screen, update_static_background}};

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(MenuState::TitleScreen)
            .add_system_set(
                SystemSet::on_enter(MenuState::TitleScreen)
                    .with_system(show_title_screen.system().chain(handle_error.system()))
            )
            .add_system_set(
                SystemSet::on_update(MenuState::TitleScreen)
                    .with_system(update_static_background.system())
            )
            .add_startup_system(load_menus.system().chain(handle_error.system()));
    }
}
