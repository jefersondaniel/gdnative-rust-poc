use bevy_ecs::prelude::*;
use gdnative::core_types::{Vector2, Transform2D};

use crate::{systems::visual_server::canvas_item::CanvasItemBundle, core::configuration::Configuration};

pub struct HudLayer {
    pub offset: Vector2,
}

pub fn setup_layers(
    mut commands: Commands,
    configuration: Res<Configuration>
) {
    let hud_layer = HudLayer {
        offset: Vector2::new(configuration.screen_size.width / 2.0, 0.0)
    };

    commands.spawn_bundle(CanvasItemBundle {
        transform: Transform2D::translation(hud_layer.offset.x, hud_layer.offset.y),
        ..Default::default()
    }).insert(hud_layer);
}
