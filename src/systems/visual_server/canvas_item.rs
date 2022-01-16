use bevy_ecs::{prelude::*};
use bevy_app::{AppBuilder, Plugin};
use bevy_transform::components::{Children, Parent};
use gdnative::{api::VisualServer, core_types::{Rect2, Rid, Transform2D, Point2, Size2, Color}};
use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::systems::visual_server::enumerations::{VisualServerStage};

use super::{material::Material, root_node::RootNode, sprite::Sprite, texture::Texture, mesh_2d::Mesh2d, text::common::Text};

#[derive(Copy, Clone)]
pub struct CanvasItem {
    pub rid: Rid,
    pub version: u64
}

pub struct CanvasItemState {
    canvas_item_rids: HashMap<u32, Rid>,
}

impl Default for CanvasItem {
    fn default() -> Self {
        CanvasItem {
            rid: Rid::new(),
            version: 0
        }
    }
}

pub struct Visible {
    pub is_visible: bool,
}

impl Default for Visible {
    fn default() -> Self { Visible { is_visible: true } }
}

#[derive(Default)]
pub struct BackBufferCopy {
    pub enabled: bool,
    pub rect: Rect2
}

#[derive(Default)]
pub struct ZIndex(pub i64);

impl From<i32> for ZIndex {
    fn from(i: i32) -> Self {
        Self(i as i64)
    }
}

#[derive(Copy, Clone, Default)]
pub struct ClipRect {
    pub rect: Rect2,
    pub global: bool
}

#[derive(Copy, Clone)]
pub struct Modulate(pub Color);

impl Default for Modulate {
    fn default() -> Self {
        return Modulate(Color::rgba(1.0, 1.0, 1.0, 1.0));
    }
}

impl ClipRect {
    pub fn global(rect: Rect2) -> Self {
        ClipRect {
            rect,
            global: true
        }
    }

    pub fn local(rect: Rect2) -> Self {
        ClipRect {
            rect,
            global: false
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct GlobalTransform(pub Transform2D);

#[derive(Default, Bundle)]
pub struct CanvasItemBundle {
    pub canvas_item: CanvasItem,
    pub visible: Visible,
    pub back_buffer_copy: BackBufferCopy,
    pub transform: Transform2D,
    pub global_transform: GlobalTransform,
    pub clip_rect: ClipRect,
    pub material: Option<Arc<RwLock<Material>>>,
    pub z_index: ZIndex,
    pub modulate: Modulate,
}

type UpdateCanvasFilter = (Added<CanvasItem>, Changed<Sprite>, Changed<Arc<Texture>>, Changed<Mesh2d>, Changed<Text>, Changed<ClipRect>);

fn update_canvas_item(
    mut canvas_item_state:  ResMut<CanvasItemState>,
    mut query: Query<
        (Entity, &mut CanvasItem, &BackBufferCopy, &Option<Arc<RwLock<Material>>>),
        Or<UpdateCanvasFilter>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        mut canvas_item,
        back_buffer_copy,
        material,
    ) in query.iter_mut() {
        if canvas_item.rid.is_valid() {
            visual_server.canvas_item_clear(canvas_item.rid);
        } else {
            canvas_item.rid = visual_server.canvas_item_create();
            canvas_item_state.canvas_item_rids.insert(entity.id(), canvas_item.rid);
        }

        canvas_item.version += 1; // Other system watches for this to know they need to update the commands

        if let Some(material) = material {
            visual_server.canvas_item_set_material(canvas_item.rid, material.read().unwrap().rid);
        }

        visual_server.canvas_item_set_copy_to_backbuffer(canvas_item.rid, back_buffer_copy.enabled, back_buffer_copy.rect);
    }
}

fn parent_canvas_item(
    root_node:  Res<RootNode>,
    canvas_item_state:  Res<CanvasItemState>,
    query: Query<(&CanvasItem, Option<&Parent>), Or<UpdateCanvasFilter>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (canvas_item, parent) in query.iter() {
        let mut parent_rid = root_node.canvas_item_rid;

        if let Some(parent) = parent {
            let parent_entity = parent.0;

            if let Some(rid) = canvas_item_state.canvas_item_rids.get(&parent_entity.id()) {
                parent_rid = *rid;
            }
        }

        visual_server.canvas_item_set_parent(canvas_item.rid, parent_rid);
    }
}

fn transform_canvas_item(
    query: Query<(Entity, &CanvasItem, &Transform2D), Or<(Changed<Transform2D>, Changed<CanvasItem>)>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, transform) in query.iter() {
        visual_server.canvas_item_set_transform(canvas_item.rid, *transform);
    }
}

fn clip_canvas_item(
    query: Query<(Entity, &CanvasItem, &GlobalTransform, &ClipRect), Or<(Changed<Transform2D>, Changed<ClipRect>, Changed<CanvasItem>)>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, global_transform, clip_rect) in query.iter() {
        if clip_rect.rect.size.width > 0.0 {
            let mut transformed_clip_rect = clip_rect.rect.clone();
            let global_position = global_transform.0.transform_point(Point2::default());
            let inverse_transform = Transform2D::translation(-global_position.x, -global_position.y);

            if clip_rect.global {
                transformed_clip_rect.origin = inverse_transform.transform_point(transformed_clip_rect.origin);
            }

            visual_server.canvas_item_set_clip(canvas_item.rid, true);
            visual_server.canvas_item_set_custom_rect(canvas_item.rid,true, transformed_clip_rect);
        }
    }
}

fn zindex_canvas_item(
    query: Query<(Entity, &CanvasItem, &ZIndex), Or<(Changed<ZIndex>, Changed<CanvasItem>)>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, z_index) in query.iter() {
        visual_server.canvas_item_set_z_index(canvas_item.rid, z_index.0);
    }
}

fn hide_canvas_item(query: Query<(Entity, &CanvasItem, &Visible), Or<(Changed<Visible>, Changed<CanvasItem>)>>) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, visible) in query.iter() {
        visual_server.canvas_item_set_visible(canvas_item.rid, visible.is_visible);
    }
}

fn modulate_canvas_item(
    query: Query<(Entity, &CanvasItem, &Modulate), Changed<Modulate>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, modulate) in query.iter() {
        visual_server.canvas_item_set_modulate(canvas_item.rid, modulate.0);
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

fn transform_propagate_system(
    mut root_query: Query<
        (Entity, Option<&Children>, &Transform2D, &mut GlobalTransform),
        Without<Parent>,
    >,
    mut transform_query: Query<(&Transform2D, &mut GlobalTransform), With<Parent>>,
    changed_transform_query: Query<Entity, Changed<Transform2D>>,
    children_query: Query<Option<&Children>, (With<Parent>, With<GlobalTransform>)>,
) {
    for (entity, children, transform, mut global_transform) in root_query.iter_mut() {
        let mut changed = false;

        if changed_transform_query.get(entity).is_ok() {
            *global_transform = GlobalTransform(*transform);
            changed = true;
        }

        if let Some(children) = children {
            for child in children.iter() {
                propagate_recursive(
                    &global_transform,
                    &changed_transform_query,
                    &mut transform_query,
                    &children_query,
                    *child,
                    changed,
                );
            }
        }
    }
}

fn propagate_recursive(
    parent: &GlobalTransform,
    changed_transform_query: &Query<Entity, Changed<Transform2D>>,
    transform_query: &mut Query<(&Transform2D, &mut GlobalTransform), With<Parent>>,
    children_query: &Query<Option<&Children>, (With<Parent>, With<GlobalTransform>)>,
    entity: Entity,
    mut changed: bool,
) {
    changed |= changed_transform_query.get(entity).is_ok();

    let global_matrix = {
        if let Ok((transform, mut global_transform)) = transform_query.get_mut(entity) {
            if changed {
                *global_transform = GlobalTransform(parent.0.then(transform));
            }
            *global_transform
        } else {
            return;
        }
    };

    if let Ok(Some(children)) = children_query.get(entity) {
        for child in children.iter() {
            propagate_recursive(
                &global_matrix,
                changed_transform_query,
                transform_query,
                children_query,
                *child,
                changed,
            );
        }
    }
}

#[derive(Default)]
pub struct CanvasItemPlugin;

impl Plugin for CanvasItemPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .insert_resource(CanvasItemState { canvas_item_rids: HashMap::new() })
            .add_system_to_stage(VisualServerStage::Remove, remove_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Remove, transform_propagate_system.system())
            .add_system_to_stage(VisualServerStage::CanvasItemUpdate, update_canvas_item.system().label("create"))
            .add_system_to_stage(VisualServerStage::CanvasItemUpdate, parent_canvas_item.system().after("create"))
            .add_system_to_stage(VisualServerStage::Transform, transform_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, clip_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, zindex_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, modulate_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, hide_canvas_item.system());
    }
}
