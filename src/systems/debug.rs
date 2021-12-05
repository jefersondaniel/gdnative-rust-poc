use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use gdnative::{api::{visual_server::TextureFlags}, core_types::{Point2, Vector2}, godot_print};

use crate::{core::{error::DataError, sprite_id::SpriteId}, drawing::{sprite_system::SpriteSystem}, systems::visual_server::{sprite::{Sprite, SpriteBundle}, text::{text_plugin::{TextBundle}, common::{TextStyle, Text, TextAlignment, HorizontalAlign}}}};

use super::{log::handle_error, visual_server::{sprite::Visible, transform::Transform, text::font_loader::FontLoader}};

fn setup(
    mut commands: Commands,
    mut font_loader: ResMut<FontLoader>,
    sprite_system: Res<SpriteSystem>
) -> Result<(), DataError> {
    godot_print!("Start debug");

    let mut sprite_file = sprite_system.get_sprite_file("res://data/data/system.sff")?;
    let sff_data = sprite_file.get_sprite(&SpriteId::new(0, 0))?;
    let texture = sff_data.create_texture(None, TextureFlags(0))?;
    let size = texture.size;
    let offset = Point2::new(sff_data.x as f32, sff_data.y as f32);

    commands.spawn_bundle(SpriteBundle {
        texture,
        sprite: Sprite {
            size,
            offset,
            flip_h: true,
            ..Default::default()
        },
        ..Default::default()
    });

    match font_loader.load_dynamic_font("res://data/inconsolata.ttf") {
        Ok(font) => {
            commands.spawn_bundle(TextBundle {
                text: Text::new(
                    "Almost before we knew it, we had left the ground.\nHello world",
                    TextStyle {
                        font,
                        font_size: 32,
                        ..Default::default()
                    },
                    TextAlignment::default()
                ),
                transform: Transform::translation(Vector2::new(100.0, 100.0)),
                ..Default::default()
            });
        },
        Err(error) => {
            godot_print!("Cant load font: {}", error);
        }
    }

    Ok(())
}

#[derive(Default)]
struct Counter(i32);

fn movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Visible)>,
    mut counter: Local<Counter>
) {
    counter.0 = counter.0 + 1;

    for (_, mut transform, _) in query.iter_mut() {
        transform.translation += Vector2::new(1.0, 1.0);
    }

    if counter.0 % 5 == 0 {
        for (_, _, mut visible) in query.iter_mut() {
            visible.is_visible = !visible.is_visible;
        }
    }

    if counter.0 > 300 {
        for (entity, _, _) in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system().chain(handle_error.system()));
        // app.add_system(movement.system());
    }
}
