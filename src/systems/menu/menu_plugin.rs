use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin, StartupStage};

use crate::{systems::log::handle_error, menus::menu_state::MenuState, core::enumerations::CombatMode};

use super::{load_menus::load_menus, title_screen_systems::TitleScreenPlugin, setup_layers::setup_layers};

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(MenuState::Title)
            .add_state(CombatMode::None)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_menus.system().chain(handle_error.system()))
            .add_startup_system(setup_layers.system())
            .add_plugin(TitleScreenPlugin::default());
    }
}
