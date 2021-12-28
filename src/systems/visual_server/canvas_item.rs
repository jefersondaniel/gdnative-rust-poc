use bevy_ecs::{prelude::*};
use bevy_app::{AppBuilder, Plugin};
use bevy_transform::components::{Children, Parent};
use gdnative::{api::VisualServer, core_types::{Rect2, Rid, Transform2D}};
use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::systems::visual_server::enumerations::{VisualServerStage};

use super::{material::Material, root_node::RootNode, sprite::Sprite, texture::Texture, mesh_2d::Mesh2d, text::common::Text};

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

pub struct ClipRect(pub Rect2);

#[derive(Copy, Clone, Default)]
pub struct GlobalTransform(pub Transform2D);

fn setup_canvas_item(
    visual_server: &VisualServer,
    parent_canvas_item: &Option<CanvasItem>,
    entity: &Entity,
    root_node: &Res<RootNode>,
    canvas_item_state: &mut ResMut<CanvasItemState>,
    canvas_item: &mut CanvasItem,
    transform: &Transform2D,
    visible: &Visible,
    back_buffer_copy: &BackBufferCopy,
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

    visual_server.canvas_item_set_copy_to_backbuffer(canvas_item.rid, back_buffer_copy.enabled, back_buffer_copy.rect);
    visual_server.canvas_item_set_transform(canvas_item.rid, *transform);
    visual_server.canvas_item_set_visible(canvas_item.rid, visible.is_visible);

    if let Some(parent_canvas_item) = parent_canvas_item {
        visual_server.canvas_item_set_parent(canvas_item.rid, parent_canvas_item.rid);
    } else {
        visual_server.canvas_item_set_parent(canvas_item.rid, root_node.canvas_item_rid);
    }
}

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut canvas_item_state:  ResMut<CanvasItemState>,
    parents_query: Query<(Entity, Option<&Children>), With<CanvasItem>>,
    mut query: Query<
        (&mut CanvasItem, &Transform2D, &Visible, &BackBufferCopy, &Option<Arc<RwLock<Material>>>, &Option<ClipRect>),
        Or<(Changed<Sprite>, Changed<Arc<Texture>>, Changed<Option<ClipRect>>, Changed<Mesh2d>, Changed<Text>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (parent, children) in parents_query.iter() {
        let mut parent_canvas_item: Option<CanvasItem> = None;

        if let Ok((
            mut canvas_item,
            transform,
            visible,
            back_buffer_copy,
            material,
            clip_rect
        )) = query.get_mut(parent) {
            setup_canvas_item(
                visual_server,
                &None,
                &parent,
                &root_node,
                &mut canvas_item_state,
                &mut canvas_item,
                transform,
                visible,
                back_buffer_copy,
                material,
                clip_rect,
            );

            parent_canvas_item = Some(canvas_item.clone());
        }

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok((
                    mut canvas_item,
                    transform,
                    visible,
                    back_buffer_copy,
                    material,
                    clip_rect
                )) = query.get_mut(*child) {
                    setup_canvas_item(
                        visual_server,
                        &parent_canvas_item,
                        &parent,
                        &root_node,
                        &mut canvas_item_state,
                        &mut canvas_item,
                        transform,
                        visible,
                        back_buffer_copy,
                        material,
                        clip_rect,
                    );
                }
            }
        }
    }
}

fn transform_canvas_item(
    query: Query<(Entity, &CanvasItem, &Transform2D), Changed<Transform2D>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, transform) in query.iter() {
        visual_server.canvas_item_set_transform(canvas_item.rid, *transform);
    }
}

fn zindex_canvas_item(
    query: Query<(Entity, &CanvasItem, &ZIndex), Changed<ZIndex>>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (_, canvas_item, z_index) in query.iter() {
        visual_server.canvas_item_set_z_index(canvas_item.rid, z_index.0);
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
            .add_system_to_stage(VisualServerStage::CanvasItemUpdate, update_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, transform_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, zindex_canvas_item.system())
            .add_system_to_stage(VisualServerStage::Transform, hide_canvas_item.system());
    }
}
