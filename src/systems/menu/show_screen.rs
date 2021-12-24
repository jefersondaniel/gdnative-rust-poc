use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::BuildChildren;

use crate::{core::{error::DataError, configuration::Configuration}, menus::title_screen::TitleScreen};

#[derive(Default)]
struct Screen;

pub fn show_title_screen(
    mut commands: Commands,
    title_screen: Res<TitleScreen>,
    configuration: Res<Configuration>
) -> Result<(), DataError> {
    let background_group = &title_screen.non_combat_screen.background_group;

    commands.spawn()
        .insert(Screen::default())
        .with_children(|parent| {
            parent.spawn().insert(background_group.clone());

            background_group.render(parent, &configuration);
        });

    Ok(())
}
