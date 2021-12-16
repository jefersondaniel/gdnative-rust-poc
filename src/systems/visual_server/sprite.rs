use crate::systems::visual_server::canvas_item::build_canvas_item;
use crate::systems::visual_server::canvas_item::Visible;
use std::{collections::HashMap, sync::{Arc, RwLock}};

use bevy_app::{AppBuilder, Plugin};
use bevy_ecs::prelude::*;
use gdnative::core_types::Transform2D;
use gdnative::{api::VisualServer, core_types::{Color, Point2, Rect2, Rid, Size2}};

use crate::systems::visual_server::enumerations::{VisualServerStage};

use super::canvas_item::ClipRect;
use super::{root_node::RootNode, texture::Texture, material::Material};

#[derive(Default)]
pub struct Sprite {
    pub size: Size2,
    pub rect: Option<Rect2>,
    pub offset: Point2,
    pub flip_v: bool,
    pub flip_h: bool,
}

#[derive(Bundle)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub texture: Arc<Texture>,
    pub visible: Visible,
    pub transform: Transform2D,
    pub clip_rect: Option<ClipRect>,
    pub material: Option<Arc<RwLock<Material>>>,
}

impl Default for SpriteBundle {
    fn default() -> Self {
        SpriteBundle {
            sprite: Sprite::default(),
            texture: Arc::new(Texture::invalid()),
            visible: Visible::default(),
            transform: Transform2D::default(),
            clip_rect: None,
            material: None,
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
        (Entity, &Sprite, &Arc<Texture>, &Transform2D, &Visible, &Option<Arc<RwLock<Material>>>, &Option<ClipRect>),
        Or<(Changed<Sprite>, Changed<Arc<Texture>>, Changed<Option<ClipRect>>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        sprite,
        texture,
        transform,
        visible,
        material,
        clip_rect
    ) in query.iter() {
        let rid = build_canvas_item(&visual_server, entity.id(), &mut rid_map.canvas_items);

        let mut dst_rect = Rect2::new(
            sprite.offset,
            sprite.size
        );

        let src_rect = match sprite.rect {
            Some(rect) => rect,
            None => Rect2::new(Point2::default(), texture.size)
        };

        if sprite.flip_v {
            dst_rect.size.height = -dst_rect.size.height;
        }

        if sprite.flip_h {
            dst_rect.size.width = -dst_rect.size.width;
        }

        if let Some(material) = material {
            visual_server.canvas_item_set_material(rid, material.read().unwrap().rid);
        }

        if let Some(clip_rect) = clip_rect {
            visual_server.canvas_item_set_clip(rid, true);
            visual_server.canvas_item_set_custom_rect(rid,true, clip_rect.0);
        }

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

        visual_server.canvas_item_set_transform(rid, *transform);
        visual_server.canvas_item_set_visible(rid, visible.is_visible);

        // Only attach to parent after all changes
        visual_server.canvas_item_set_parent(rid, root_node.canvas_item_rid);
    }
}

fn transform_canvas_item(
    rid_map: Res<RidMap>,
    query: Query<(Entity, &Transform2D), (Changed<Transform2D>, With<Sprite>)>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, transform) in query.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            visual_server.canvas_item_set_transform(*rid, *transform);
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
