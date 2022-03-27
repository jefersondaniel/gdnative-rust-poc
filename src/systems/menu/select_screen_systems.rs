use bevy_app::{AppBuilder, Plugin, EventWriter};
use bevy_ecs::prelude::*;
use bevy_transform::{hierarchy::{BuildChildren, DespawnRecursiveExt}, components::Parent};
use gdnative::{core_types::{Transform2D, Point2}, api::visual_server::TextureFlags};

use crate::{menus::{menu_state::MenuState, select_screen::SelectScreen}, systems::{backgrounds::events::BackgroundGroupEvent, visual_server::{canvas_item::CanvasItemBundle, sprite::{SpriteBundle, Sprite}}}, core::{constants, sprite_id::SpriteId, configuration::{Configuration, ScaleForScreen}, enumerations::PlayerSelectType}, profiles::profile_loader::ProfileLoader};

use super::{setup_layers::HudLayer, components::MenuSpriteFile};

#[derive(Default)]
pub struct SelectScreenPlugin;

#[derive(Default)]
struct ScreenMarker;

fn show_screen(
    mut commands: Commands,
    mut background_group_event: EventWriter<BackgroundGroupEvent>,
    mut profile_loader: ResMut<ProfileLoader>,
    mut menu_sprite_file: ResMut<MenuSpriteFile>,
    configuration: Res<Configuration>,
    hud_layer_query: Query<Entity, With<HudLayer>>,
    select_screen: Res<SelectScreen>
) {
    let hud_entity = hud_layer_query.single().expect("HudLayer not found");
    let background_group = &select_screen.non_combat_screen.background_group;
    let mut menu_sprite_file = menu_sprite_file.0.write().expect("Unable to read menu sprite file");

    let screen_entity = commands.spawn_bundle(CanvasItemBundle::default())
        .insert(ScreenMarker::default())
        .id();

    let background_layer = commands.spawn_bundle(CanvasItemBundle::default())
        .insert(Parent(screen_entity))
        .id();

    background_group_event.send(BackgroundGroupEvent {
        layer: background_layer.clone(),
        background_group: background_group.clone(),
    });

    for y in 0..select_screen.rows {
        for x in 0..select_screen.columns {
            let mut position = select_screen.grid_position;
            position.x += (select_screen.cellsize.x + select_screen.cellspacing as f32) * x as f32;
            position.y += (select_screen.cellsize.y + select_screen.cellspacing as f32) * y as f32;

            select_screen.cellbg.render(
                &mut commands,
                &mut menu_sprite_file,
                position
            ).insert(Parent(screen_entity));

            if let Some(select) = profile_loader.get_player_on_grid((x, y)) {
                if select.select_type == PlayerSelectType::Random {
                    select_screen.cellrandom.render(
                        &mut commands,
                        &mut menu_sprite_file,
                        position
                    ).insert(Parent(screen_entity));
                } else {
                    let profile = select.profile.unwrap();
                    let mut sprite_file = profile.sprite_file.write().unwrap();
                    if let Ok(small_portrait) = sprite_file.get_sprite(&SpriteId::SMALL_PORTRAIT) {
                        let small_portrait_texture = small_portrait.create_texture(None, TextureFlags(0)).unwrap();

                        commands.spawn_bundle(SpriteBundle {
                            texture: small_portrait_texture.clone(),
                            sprite: Sprite {
                                size: small_portrait_texture.size.scale_for_screen(&configuration, profile.localcoord),
                                offset: small_portrait.offset().scale_for_screen(&configuration, profile.localcoord),
                                ..Default::default()
                            },
                            transform: Transform2D::translation(position.x, position.y),
                            ..Default::default()
                        }).insert(Parent(screen_entity));
                    }
                }
            }
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
