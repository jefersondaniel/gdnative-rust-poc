use bevy_app::{AppBuilder, Plugin, EventWriter};
use bevy_ecs::prelude::*;
use bevy_transform::{hierarchy::{BuildChildren, DespawnRecursiveExt}, components::Parent};
use gdnative::core_types::Transform2D;

use crate::{menus::{menu_state::MenuState, select_screen::SelectScreen}, systems::{backgrounds::events::BackgroundGroupEvent, visual_server::{canvas_item::CanvasItemBundle, sprite::{SpriteBundle, Sprite}}}, core::constants};

use super::setup_layers::HudLayer;

#[derive(Default)]
pub struct SelectScreenPlugin;

#[derive(Default)]
struct ScreenMarker;

fn show_screen(
    mut commands: Commands,
    mut background_group_event: EventWriter<BackgroundGroupEvent>,
    hud_layer_query: Query<Entity, With<HudLayer>>,
    select_screen: Res<SelectScreen>
) {
    let hud_entity = hud_layer_query.single().expect("HudLayer not found");
    let background_group = &select_screen.non_combat_screen.background_group;

    let screen_entity = commands.spawn_bundle(CanvasItemBundle::default())
        .insert(ScreenMarker::default())
        .id();

    background_group_event.send(BackgroundGroupEvent {
        layer: screen_entity.clone(),
        background_group: background_group.clone(),
    });

    let cellbg = &select_screen.cellbg;

    for y in 0..select_screen.rows {
        for x in 0..select_screen.columns {
            let mut location = select_screen.grid_position;
            location.x += (select_screen.cellsize.x + select_screen.cellspacing as f32) * x as f32;
            location.y += (select_screen.cellsize.y + select_screen.cellspacing as f32) * y as f32;

            commands.spawn_bundle(SpriteBundle {
                texture: cellbg.texture.clone(),
                sprite: Sprite {
                    size: cellbg.texture.size,
                    ..Default::default()
                },
                z_index: (constants::BG_LAYER_FRONT_Z_INDEX_MAX + 1).into(),
                transform: Transform2D::translation(location.x, location.y),
                ..Default::default()
            }).insert(Parent(screen_entity));
        }
    }

    commands.entity(hud_entity).push_children(&[screen_entity]);
}

fn hide_screen(
    mut commands: Commands,
    query: Query<Entity, With<ScreenMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for SelectScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(MenuState::Select)
                    .with_system(show_screen.system())
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Select)
                    .with_system(hide_screen.system())
            );
    }
}
