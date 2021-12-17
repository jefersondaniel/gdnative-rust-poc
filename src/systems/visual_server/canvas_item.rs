use bevy_ecs::{prelude::*};
use bevy_app::{AppBuilder, Plugin};
use gdnative::{api::VisualServer, core_types::{Rect2, Rid, Transform2D}};
use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::systems::visual_server::enumerations::{VisualServerStage};

use super::{material::Material, root_node::RootNode};

#[derive(Copy, Clone)]
pub struct CanvasItem {
    pub rid: Rid
}

pub struct CanvasItemState {
    canvas_item_rids: HashMap<u32, Rid>,
}

impl Default for CanvasItem {
    fn default() -> Self {
        CanvasItem {
            rid: Rid::new(),
        }
    }
}

pub struct Visible {
    pub is_visible: bool,
}

impl Default for Visible {
    fn default() -> Self { Visible { is_visible: true } }
}

pub struct ClipRect(pub Rect2);

pub fn setup_canvas_item(
    entity: &Entity,
    visual_server: &VisualServer,
    root_node: &Res<RootNode>,
    canvas_item_state: &mut ResMut<CanvasItemState>,
    canvas_item: &mut CanvasItem,
    transform: &Transform2D,
    visible: &Visible,
    material: &Option<Arc<RwLock<Material>>>,
    clip_rect: &Option<ClipRect>
) {
    if canvas_item.rid.is_valid() {
        visual_server.canvas_item_clear(canvas_item.rid);
    } else {
        canvas_item.rid = visual_server.canvas_item_create();
        canvas_item_state.canvas_item_rids.insert(entity.id(), canvas_item.rid);
    }

    if let Some(material) = material {
        visual_server.canvas_item_set_material(canvas_item.rid, material.read().unwrap().rid);
    }

    if let Some(clip_rect) = clip_rect {
        visual_server.canvas_item_set_clip(canvas_item.rid, true);
        visual_server.canvas_item_set_custom_rect(canvas_item.rid,true, clip_rect.0);
    }

    visual_server.canvas_item_set_transform(canvas_item.rid, *transform);
    visual_server.canvas_item_set_visible(canvas_item.rid, visible.is_visible);
    visual_server.canvas_item_set_parent(canvas_item.rid, root_node.canvas_item_rid);
}

fn transform_canvas_item(
    query: Query<(Entity, &CanvasItem, &Transform2D), Changed<Transform2D>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, transform) in query.iter() {
        visual_server.canvas_item_set_transform(canvas_item.rid, *transform);
    }
}

fn hide_canvas_item(query: Query<(Entity, &CanvasItem, &Visible), Changed<Visible>>) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, visible) in query.iter() {
        visual_server.canvas_item_set_visible(canvas_item.rid, visible.is_visible);
    }
}

fn remove_canvas_item(
    mut canvas_item_state: ResMut<CanvasItemState>,
    removals: RemovedComponents<CanvasItem>,
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };
    let mut affected_entity_ids = Vec::new();

    for entity in removals.iter() {
        if let Some(rid) = canvas_item_state.canvas_item_rids.get(&entity.id()) {
            affected_entity_ids.push(entity.id());
            visual_server.free_rid(*rid);
        }
    }

    for id in affected_entity_ids.iter() {
        canvas_item_state.canvas_item_rids.remove(id);
    }
}

#[derive(Default)]
pub struct CanvasItemPlugin;

impl Plugin for CanvasItemPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .insert_resource(CanvasItemState { canvas_item_rids: HashMap::new() })
            .add_system_to_stage(VisualServerStage::Remove, remove_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, transform_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, hide_canvas_item.system());
    }
}
