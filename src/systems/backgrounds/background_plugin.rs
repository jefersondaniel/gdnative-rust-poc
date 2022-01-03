use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_app::{AppBuilder, Plugin, EventReader};
use gdnative::core_types::{Transform2D};

use crate::{core::{configuration::Configuration, constants::{BG_LAYER_BACK_Z_INDEX_MIN, BG_LAYER_FRONT_Z_INDEX_MIN}, enumerations::BackgroundLayer}, backgrounds::static_background::StaticBackground, systems::visual_server::canvas_item::CanvasItemBundle};

use super::events::BackgroundGroupEvent;

fn show_background_group(
    mut commands: Commands,
    mut events: EventReader<BackgroundGroupEvent>,
    configuration: Res<Configuration>
) {
    for event in events.iter() {
        commands
            .entity(event.layer)
            .with_children(|parent_builder| {
                parent_builder.spawn_bundle(CanvasItemBundle {
                    transform: Transform2D::translation(configuration.screen_size.width / 2.0, 0.0),
                    ..Default::default()
                }).with_children(|child_builder| {
                    let mut back_z_index = BG_LAYER_BACK_Z_INDEX_MIN;
                    let mut front_z_index = BG_LAYER_FRONT_Z_INDEX_MIN;

                    for background in event.background_group.backgrounds.iter() {
                        let z_index = match background.layer() {
                            BackgroundLayer::Back => back_z_index,
                            BackgroundLayer::Front => front_z_index,
                        };
                        background.render(child_builder, &configuration, z_index.clone());
                        match background.layer() {
                            BackgroundLayer::Back => back_z_index = back_z_index + 1,
                            BackgroundLayer::Front => front_z_index = front_z_index + 1,
                        };
                    }
                });
            });
    }
}

fn update_static_background(mut query: Query<(&StaticBackground, &mut Transform2D)>) {
    for (background, transform) in query.iter_mut() {
        background.update(transform);
    }
}

#[derive(Default)]
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<BackgroundGroupEvent>()
            .add_system(show_background_group.system())
            .add_system(update_static_background.system());
    }
}
