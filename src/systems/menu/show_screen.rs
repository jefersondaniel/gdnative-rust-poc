use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_transform::{hierarchy::BuildChildren, components::Parent};
use gdnative::core_types::{Transform2D, Vector2, Point2};

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
    mut query: Query<(&StaticBackground, &mut Transform2D)>,
) {
    for (
        background,
        mut transform
    ) in query.iter_mut() {
        let velocity = background.base_background.velocity;

        if velocity == Vector2::new(0.0, 0.0) {
            continue;
        }

        *transform = transform.then_translate(velocity);

        let size = background.sprite.size;
        let location = transform.transform_point(Point2::default());
        let startlocation = background.base_background.startlocation;

        if location.x >= startlocation.x + size.width || location.x <= startlocation.x - size.width {
            *transform = transform
                .then_translate(Vector2::new(-location.x, 0.0))
                .then_translate(Vector2::new(startlocation.x, 0.0));
        }

        if location.y >= startlocation.y + size.height || location.y <= startlocation.y - size.height {
            *transform = transform
                .then_translate(Vector2::new(0.0, -location.y))
                .then_translate(Vector2::new(0.0, startlocation.y));
        }
    }
}
