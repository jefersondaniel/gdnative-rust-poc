use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin, StartupStage};

use crate::{systems::log::handle_error, menus::menu_state::MenuState};

use super::{load_menus::load_menus, show_title_screen::{show_title_screen}, setup_layers::setup_layers};

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(MenuState::TitleScreen)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_menus.system().chain(handle_error.system()))
            .add_startup_system(setup_layers.system())
            .add_system_set(
                SystemSet::on_enter(MenuState::TitleScreen)
                    .with_system(show_title_screen.system())
            );
    }
}
