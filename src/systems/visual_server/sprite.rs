use std::{collections::HashMap, sync::Arc};

use bevy_app::{AppBuilder, Plugin};
use bevy_ecs::prelude::*;
use gdnative::{api::VisualServer, core_types::{Color, Point2, Rect2, Rid, Size2}};

use crate::systems::visual_server::enumerations::{VisualServerLabel, VisualServerStage};

use super::{root_node::RootNode, texture::Texture, transform::Transform};

#[derive(Default)]
pub struct Sprite {
    pub size: Size2,
    pub offset: Point2,
    pub flip_v: bool,
    pub flip_h: bool,
}

pub struct Visible {
    pub is_visible: bool,
}

impl Default for Visible {
    fn default() -> Self { Visible { is_visible: true } }
}

#[derive(Bundle)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub texture: Arc<Texture>,
    pub visible: Visible,
    pub transform: Transform,
}

impl Default for SpriteBundle {
    fn default() -> Self {
        SpriteBundle {
            sprite: Sprite::default(),
            texture: Arc::new(Texture::invalid()),
            visible: Visible::default(),
            transform: Transform::default(),
        }
    }
}

struct SpriteRidMap {
    pub canvas_items: HashMap<u32, Rid>,
}

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut rid_map: ResMut<SpriteRidMap>,
    query: Query<
        (Entity, &Sprite, &Arc<Texture>, &Transform),
        Or<(Changed<Sprite>, Changed<Arc<Texture>>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        sprite,
        texture,
        transform
    ) in query.iter() {
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

        gdnative::godot_print!("update canvas item: {}", rid.get_id());

        let src_rect = Rect2::new(
            Point2::new(0.0, 0.0),
            texture.size
        );

        let mut dst_rect = Rect2::new(
            sprite.offset,
            sprite.size
        );

        if sprite.flip_v {
            dst_rect.size.height = -dst_rect.size.height;
        }

        if sprite.flip_h {
            dst_rect.size.width = -dst_rect.size.width;
        }

        visual_server.canvas_item_set_parent(rid, root_node.canvas_item_rid);
        visual_server.canvas_item_add_texture_rect_region(
            rid,
            dst_rect,
            texture.rid,
            src_rect,
            Color::rgba(1.0, 1.0, 1.0, 1.0),
            false,
            Rid::new(),
            false
        );
        visual_server.canvas_item_set_transform(rid, transform.into());
    }
}

fn transform_canvas_item(
    mut rid_map: ResMut<SpriteRidMap>,
    query: Query<(Entity, &Transform), Changed<Transform>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, transform) in query.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            gdnative::godot_print!("transform canvas item: {}", rid.get_id());

            visual_server.canvas_item_set_transform(*rid, transform.into());
        }
    }
}

#[derive(Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .insert_resource(SpriteRidMap { canvas_items: HashMap::new() })
            .add_system_to_stage(VisualServerStage::Update, update_canvas_item.system().label(VisualServerLabel::Update))
            .add_system_to_stage(VisualServerStage::Update, transform_canvas_item.system().label(VisualServerLabel::Transform).after(VisualServerLabel::Update));
    }
}
