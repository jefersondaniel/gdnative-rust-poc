use bevy_ecs::prelude::*;
use gdnative::core_types::{Vector2};

use crate::{systems::visual_server::canvas_item::CanvasItemBundle};

pub struct HudLayer {
    pub offset: Vector2,
}

pub fn setup_layers(mut commands: Commands) {
    let hud_layer = HudLayer {
        offset: Vector2::new(0.0, 0.0)
    };

    commands.spawn_bundle(CanvasItemBundle::default()).insert(hud_layer);
}
