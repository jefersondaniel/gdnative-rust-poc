use std::{collections::HashMap, sync::Arc};

use bevy_app::{AppBuilder, Plugin};
use bevy_ecs::prelude::*;
use gdnative::{api::VisualServer, core_types::{Color, Point2, Rect2, Rid, Size2}};

use crate::systems::visual_server::enumerations::{VisualServerStage};

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

struct RidMap {
    pub canvas_items: HashMap<u32, Rid>,
}

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut rid_map: ResMut<RidMap>,
    query: Query<
        (Entity, &Sprite, &Arc<Texture>, &Transform, &Visible),
        Or<(Changed<Sprite>, Changed<Arc<Texture>>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        sprite,
        texture,
        transform,
        visible
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
        visual_server.canvas_item_set_visible(rid, visible.is_visible);
    }
}

fn transform_canvas_item(
    rid_map: Res<RidMap>,
    query: Query<(Entity, &Transform), (Changed<Transform>, With<Sprite>)>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, transform) in query.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            visual_server.canvas_item_set_transform(*rid, transform.into());
        }
    }
}

fn hide_canvas_item(
    rid_map: Res<RidMap>,
    query: Query<(Entity, &Visible), (Changed<Visible>, With<Sprite>)>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, visible) in query.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            visual_server.canvas_item_set_visible(*rid, visible.is_visible);
        }
    }
}

fn remove_canvas_item(
    mut rid_map: ResMut<RidMap>,
    removals: RemovedComponents<Sprite>,
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };
    let mut affected_entity_ids = Vec::new();

    for entity in removals.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            affected_entity_ids.push(entity.id());
            visual_server.free_rid(*rid);
        }
    }

    for id in affected_entity_ids.iter() {
        rid_map.canvas_items.remove(id);
    }
}

#[derive(Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .insert_resource(RidMap { canvas_items: HashMap::new() })
            .add_system_to_stage(VisualServerStage::Remove, remove_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Update, update_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, transform_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, hide_canvas_item.system());
    }
}
