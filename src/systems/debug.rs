use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use gdnative::{api::{visual_server::{TextureFlags, PrimitiveType}, SurfaceTool}, core_types::{Point2, Vector2, Color, Rect2, Size2, Vector3, Transform2D, ToVariant}, godot_print};

use crate::{core::{error::DataError, sprite_id::SpriteId}, drawing::{sprite_system::SpriteSystem}, systems::visual_server::{sprite::{Sprite, SpriteBundle}, text::{text_plugin::{TextBundle}, common::{TextStyle, Text, TextAlignment, HorizontalAlign}, font_loader::load_dynamic_font}, shader::Shader, material::Material, mesh_2d::{Mesh2dBundle, Mesh2d}, canvas_item::{ClipRect, ZIndex}}, audio::{snd_parser::read_sounds, structs::WavSound}, io::file_system::FileSystem};

use super::{log::handle_error, visual_server::{canvas_item::{Visible}}, input::Input, audio_server::audio::Audio};

struct TestComponent;

fn setup(
    mut commands: Commands,
    sprite_system: Res<SpriteSystem>,
    file_system: Res<FileSystem>
) -> Result<(), DataError> {
    godot_print!("Start debug");

    let mut sprite_file = sprite_system.get_sprite_file("res://data/data/system.sff")?;
    let sff_data = sprite_file.get_sprite(&SpriteId::new(0, 0))?;
    let texture = sff_data.create_monochromatic_texture(TextureFlags(0));
    let palette_texture = sff_data.create_palette_texture(None);
    let size = texture.size;
    let offset = Point2::new(sff_data.x as f32, sff_data.y as f32);
    let sprite_shader_code = file_system.open_file_as_string("res://resources/sprite.glsl")?;
    let shader = Shader::allocate(&sprite_shader_code);
    let material = Material::allocate(shader);
    let mut material_write = material.write().expect("Could not lock material");
    material_write.set_shader_texture("palette", palette_texture.clone());
    material_write.set_shader_param("blend_type", 1.to_variant());
    material_write.set_shader_param("blend_source", 1.0.to_variant());
    material_write.set_shader_param("blend_destination", 1.0.to_variant());
    let texture_with_palette = sff_data.create_texture(None, TextureFlags(0))?;

    commands.spawn_bundle(SpriteBundle {
        texture: texture.clone(),
        sprite: Sprite {
            size: size * 2.0,
            offset,
            flip_h: false,
            ..Default::default()
        },
        // clip_rect: Some(ClipRect(Rect2::new(Point2::new(0.0, 0.0), Size2::new(300.0, 300.0)))),
        transform: Transform2D::translation(100.0, 0.0),
        material: Some(material.clone()),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
            texture: texture_with_palette.clone(),
            sprite: Sprite {
                size: size * 2.0,
                offset,
                flip_h: false,
                ..Default::default()
            },
            transform: Transform2D::translation(0.0, 100.0),
            material: None,
            ..Default::default()
        })
        .insert(TestComponent);
    });

    let st = SurfaceTool::new();

    st.begin(PrimitiveType::TRIANGLES.0);
    // First Triangle
    st.add_uv(Vector2::new(1.0, 0.0)); // Top Left
    st.add_vertex(Vector3::new(-80.0, 0.0, 0.0));
    st.add_uv(Vector2::new(0.0, 0.0)); // Top Right
    st.add_vertex(Vector3::new(180.0, 0.0, 0.0));
    st.add_uv(Vector2::new(0.0, 1.0)); // Bottom Right
    st.add_vertex(Vector3::new(100.0, 100.0, 0.0));
    // Second Triangle
    st.add_uv(Vector2::new(1.0, 0.0)); // Top Left
    st.add_vertex(Vector3::new(-80.0, 0.0, 0.0));
    st.add_uv(Vector2::new(1.0, 1.0)); // Bottom Left
    st.add_vertex(Vector3::new(0.0, 100.0, 0.0));
    st.add_uv(Vector2::new(0.0, 1.0)); // Bottom Right
    st.add_vertex(Vector3::new(100.0, 100.0, 0.0));

    commands.spawn_bundle(Mesh2dBundle {
        texture: texture.clone(),
        mesh: Mesh2d {
            primitive_type: PrimitiveType::TRIANGLES,
            surface_array: st.commit_to_arrays(),
        },
        transform: Transform2D::translation(100.0, 100.0),
        material: Some(material.clone()),
        // clip_rect: Some(ClipRect(Rect2::new(Point2::new(-80.0, 0.0), Size2::new(400.0, 40.0)))),
        z_index: 10.into(),
        ..Default::default()
    });

    // let bitmap_font = sprite_system.load_font("res://data/font/arcade.def")?;

    // commands.spawn_bundle(TextBundle {
    //     text: Text::new(
    //         "ABC TEST\nSecond Line",
    //         TextStyle {
    //             font: bitmap_font.get_color_bank(1),
    //             font_size: bitmap_font.size,
    //             ..Default::default()
    //         },
    //         TextAlignment::default()
    //     ),
    //     transform: Transform2D::translation(100.0, 100.0),
    //     ..Default::default()
    // });

    // let sounds = read_sounds("res://data/data/system.snd").expect("Can't read system sound");
    // commands.insert_resource(sounds);

    match load_dynamic_font("res://resources/roboto.ttf") {
        Ok(font) => {
            commands.spawn_bundle(TextBundle {
                text: Text::new(
                    "ABC 123",
                    TextStyle {
                        font,
                        font_size: 32,
                        color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                        ..Default::default()
                    },
                    TextAlignment::default()
                ),
                z_index: ZIndex(10),
                transform: Transform2D::translation(100.0, 100.0),
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
    input: Res<Input>,
    mut audio: ResMut<Audio>,
    // sounds: Res<Vec<WavSound>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform2D, &mut Visible), With<Sprite>>,
    mut counter: Local<Counter>
) {
    for (_, mut transform, _) in query.iter_mut() {
        if input.pressed("P1_B") {
            *transform = transform.then(&Transform2D::translation(-10.0, 0.0));
        }
        if input.pressed("P1_F") {
            *transform = transform.then(&Transform2D::translation(10.0, 0.0));
        }
        if input.pressed("P1_U") {
            *transform = transform.then(&Transform2D::translation(0.0, -10.0));
        }
        if input.pressed("P1_D") {
            *transform = transform.then(&Transform2D::translation(0.0, 10.0));
        }
   }

//    if counter.0 % 10 == 0 {
//        let stream = sounds[0].stream.clone();
//        audio.play(stream);
//    }

    // if counter.0 % 5 == 0 {
    //     for (_, _, mut visible) in query.iter_mut() {
    //         visible.is_visible = !visible.is_visible;
    //     }
    // }

    // if counter.0 > 600 {
    //     for (entity, _, _) in query.iter_mut() {
    //         commands.entity(entity).despawn();
    //     }
    // }

    counter.0 = counter.0 + 1;
}

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system().chain(handle_error.system()));
        app.add_system(movement.system());
    }
}
