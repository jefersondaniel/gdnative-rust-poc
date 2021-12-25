use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_transform::{hierarchy::BuildChildren, components::Parent};
use gdnative::core_types::{Transform2D, Vector2};

use crate::{core::{error::DataError, configuration::Configuration}, menus::title_screen::TitleScreen, backgrounds::{background::Background, static_background::StaticBackground}, systems::visual_server::texture::Texture};

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
            for background in background_group.backgrounds.iter() {
                background.render(parent, &configuration);
            }
        });

    Ok(())
}

pub fn update_static_background(
    mut query: Query<(&mut StaticBackground, &mut Transform2D)>,
) {
    for (
        mut background,
        mut transform
    ) in query.iter_mut() {
        *transform = transform.then_translate(Vector2::new(10.0, 10.0));
    }
}
