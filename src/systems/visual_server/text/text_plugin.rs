use std::{collections::HashMap, sync::Arc};

use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};
use gdnative::{core_types::{Color, Rid, Vector2, Size2, Rect2, Point2}};
use gdnative::api::{VisualServer};

use crate::{systems::visual_server::{enumerations::VisualServerStage, root_node::RootNode, transform::Transform, texture::Texture}, io::file_system::FileSystem, core::error::DataError};

use super::{common::FontSpacing, vector_font::VectorFont};

#[derive(Default)]
pub struct Text(String);

impl Text {
    pub fn new(text: &str) -> Self { Text(text.to_string()) }
}

pub struct Scale(i32);

impl Default for Scale {
    fn default() -> Self { Scale(18) }
}

pub enum Font {
    None,
    VectorFont {
        font: VectorFont
    }
}

impl Default for Font {
    fn default() -> Self { Font::None }
}

struct CharDrawing {
    rect: Rect2,
    current: char,
}

impl Font {
    fn draw(&mut self, visual_server: &VisualServer, canvas_item: Rid, text: &str, scale: i32) {
        gdnative::godot_print!("font draw on rid: {}", canvas_item.get_id());

        let spacing = self.get_spacing(scale);
        let characters: Vec<char> = text.chars().collect();
        let mut drawing: Vec<CharDrawing> = Vec::new();
        let mut cursor: Point2 = Point2::new(0.0, 0.0);
        let mut dimension: Vector2 = Vector2::new(0.0, 0.0);

        for (i, current_char) in characters.iter().enumerate() {
            let char_size = self.get_char_size(*current_char, scale);
            drawing.push(CharDrawing {
                rect: Rect2::new(cursor.clone(), char_size.clone()),
                current: *current_char,
            });
            cursor.x += char_size.width + spacing.character;
            dimension.y = f32::max(char_size.height, dimension.y);
            dimension.x = cursor.x;
        }

        let offset = Vector2::new(100.0, 100.0); // TODO: Get property

        for character in drawing.iter() {
            let texture = self.get_texture(character.current, scale);

            if let Some(texture) = texture {
                visual_server.canvas_item_add_texture_rect(
                    canvas_item,
                    character.rect,
                    texture.rid,
                    false,
                    Color::rgba(1.0, 1.0, 1.0, 1.0),
                    false,
                    Rid::new(),
                );
            }
        }
    }

    fn get_char_size(&self, current: char, scale: i32) -> Size2 {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_char_size(
                    current,
                    scale
                )
            },
            Font::None => Size2::new(0.0, 0.0)
        }
    }

    fn get_spacing(&self, scale: i32) -> FontSpacing {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_spacing(scale)
            },
            Font::None => FontSpacing::default(),
        }
    }

    pub fn get_texture(
        &mut self,
        glyph: char,
        scale: i32,
    ) -> Option<Arc<Texture>> {
        match self {
            Font::VectorFont { font, .. } => {
                font.get_texture(glyph, scale)
            },
            Font::None => None
        }
    }
}

#[derive(Default)]
pub struct FontLoader {
}

impl FontLoader {
    pub fn load_dynamic_font(&mut self, path: &str) -> Result<Font, DataError> {
        let filesystem = FileSystem::new();
        let file = filesystem.open_file(path)?;
        let typed_array = file.get_buffer(file.get_len());
        let font_data: Vec<u8> = typed_array.read().iter().map(|v| *v).collect();

        let result = VectorFont::try_from_bytes(font_data);

        if let Ok(font) = result {
            return Ok(Font::VectorFont {
                font: font,
            })
        }

        Err(DataError::new(format!("Invalid font: {}", path)))
    }
}

struct RidMap {
    pub canvas_items: HashMap<u32, Rid>,
}

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut rid_map: ResMut<RidMap>,
    mut query: Query<
        (Entity, &Text, &mut Font, &Scale, &Transform),
        Or<(Changed<Text>, Changed<Font>, Changed<Scale>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, text, mut font, scale, transform) in query.iter_mut() {
        if scale.0 <= 0 {
            continue;
        }

        let rid = match rid_map.canvas_items.get(&entity.id()) {
            Some(value) => {
                visual_server.canvas_item_clear(*value);
                *value
            },
            None => {
                let rid = visual_server.canvas_item_create();
                rid_map.canvas_items.insert(entity.id(), rid);
                rid
            }
        };

        visual_server.canvas_item_set_parent(rid, root_node.canvas_item_rid);
        font.draw(visual_server, rid, &text.0, scale.0);
        visual_server.canvas_item_set_transform(rid, transform.into());
    }
}

#[derive(Default, Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub scale: Scale,
    pub font: Font,
    pub transform: Transform,
}

#[derive(Default)]
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .insert_resource(RidMap { canvas_items: HashMap::new() })
            .insert_resource(FontLoader::default())
            // .add_system_to_stage(VisualServerStage::Remove, remove_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Update, update_canvas_item.system());
            // .add_system_to_stage(VisualServerStage::Transform, transform_canvas_item.system())
            // .add_system_to_stage(VisualServerStage::Transform, hide_canvas_item.system());
    }
}
