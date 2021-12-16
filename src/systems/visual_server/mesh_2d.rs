use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};
use gdnative::NewRef;
use gdnative::api::visual_server::PrimitiveType;
use gdnative::core_types::{Transform2D};
use std::collections::{HashSet,HashMap};
use std::sync::RwLock;
use std::sync::Arc;
use gdnative::{api::VisualServer, core_types::{Color, VariantArray, Point2, Rect2, Rid, Size2}};

use super::canvas_item::ClipRect;
use super::{root_node::RootNode, texture::Texture};

use crate::systems::visual_server::canvas_item::build_canvas_item;
use crate::systems::visual_server::material::Material;
use crate::systems::visual_server::canvas_item::Visible;
use crate::systems::visual_server::enumerations::{VisualServerStage};

struct RidMap {
    pub canvas_items: HashMap<u32, Rid>,
    pub meshes: HashMap<u32, Rid>,
}

pub struct Mesh2d {
    pub primitive_type: PrimitiveType,
    pub surface_array: VariantArray,
}

impl Default for Mesh2d {
    fn default() -> Self {
        Mesh2d {
            primitive_type: PrimitiveType::TRIANGLES,
            surface_array: VariantArray::new_shared(),
        }
    }
}

#[derive(Bundle)]
pub struct Mesh2dBundle {
    pub mesh: Mesh2d,
    pub texture: Arc<Texture>,
    pub visible: Visible,
    pub transform: Transform2D,
    pub material: Option<Arc<RwLock<Material>>>,
    pub clip_rect: Option<ClipRect>,
}

impl Default for Mesh2dBundle {
    fn default() -> Self {
        Mesh2dBundle {
            mesh: Mesh2d::default(),
            texture: Arc::new(Texture::invalid()),
            visible: Visible::default(),
            transform: Transform2D::default(),
            material: None,
            clip_rect: None
        }
    }
}

fn update_meshes(
    root_node: Res<RootNode>,
    mut rid_map: ResMut<RidMap>,
    query: Query<
        (Entity, &Mesh2d, &Arc<Texture>, &Transform2D, &Visible, &Option<Arc<RwLock<Material>>>, &Option<ClipRect>),
        Or<(Changed<Mesh2d>, Changed<Arc<Texture>>, Changed<Option<ClipRect>>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        mesh_2d,
        texture,
        transform,
        visible,
        material,
        clip_rect
    ) in query.iter() {
        let mesh_rid = match rid_map.meshes.get(&entity.id()) {
            Some(value) => {
                visual_server.mesh_clear(*value);
                *value
            },
            None => {
                let rid = visual_server.mesh_create();
                rid_map.meshes.insert(entity.id(), rid);
                rid
            }
        };

        visual_server.mesh_add_surface_from_arrays(
            mesh_rid,
            mesh_2d.primitive_type.0,
            mesh_2d.surface_array.new_ref(),
            VariantArray::new_shared(),
            0
        );

        let canvas_item_rid = build_canvas_item(&visual_server, entity.id(), &mut rid_map.canvas_items);
        let xform = Transform2D::translation(0.0, 0.0);
        let modulate = Color::rgba(1.0, 1.0, 1.0, 1.0);

        visual_server.canvas_item_add_mesh(
            canvas_item_rid,
            mesh_rid,
            xform.into(),
            modulate,
            texture.rid,
            Rid::new()
        );

        if let Some(material) = material {
            visual_server.canvas_item_set_material(canvas_item_rid, material.read().unwrap().rid);
        }

        if let Some(clip_rect) = clip_rect {
            visual_server.canvas_item_set_clip(canvas_item_rid, true);
            visual_server.canvas_item_set_custom_rect(canvas_item_rid,true, clip_rect.0);
        }

        visual_server.canvas_item_set_transform(canvas_item_rid, *transform);
        visual_server.canvas_item_set_visible(canvas_item_rid, visible.is_visible);
        // Only attach to parent after all changes
        visual_server.canvas_item_set_parent(canvas_item_rid, root_node.canvas_item_rid);
    }
}


fn transform_meshes(
    rid_map: Res<RidMap>,
    query: Query<(Entity, &Transform2D), (Changed<Transform2D>, With<Mesh2d>)>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, transform) in query.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            visual_server.canvas_item_set_transform(*rid, *transform);
        }
    }
}

fn hide_meshes(
    rid_map: Res<RidMap>,
    query: Query<(Entity, &Visible), (Changed<Visible>, With<Mesh2d>)>
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, visible) in query.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            visual_server.canvas_item_set_visible(*rid, visible.is_visible);
        }
    }
}

fn remove_meshes(
    mut rid_map: ResMut<RidMap>,
    removals: RemovedComponents<Mesh2d>,
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };
    let mut affected_entity_ids = HashSet::new();

    for entity in removals.iter() {
        if let Some(rid) = rid_map.canvas_items.get(&entity.id()) {
            affected_entity_ids.insert(entity.id());
            visual_server.free_rid(*rid);
        }

        if let Some(rid) = rid_map.meshes.get(&entity.id()) {
            affected_entity_ids.insert(entity.id());
            visual_server.free_rid(*rid);
        }
    }

    for id in affected_entity_ids.iter() {
        rid_map.canvas_items.remove(id);
        rid_map.meshes.remove(id);
    }
}

#[derive(Default)]
pub struct Mesh2dPlugin;

impl Plugin for Mesh2dPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .insert_resource(RidMap { canvas_items: HashMap::new(), meshes: HashMap::new() })
            .add_system_to_stage(VisualServerStage::Remove, remove_meshes.system())
            .add_system_to_stage(VisualServerStage::Update, update_meshes.system())
            .add_system_to_stage(VisualServerStage::Transform, transform_meshes.system())
            .add_system_to_stage(VisualServerStage::Transform, hide_meshes.system());
    }
}
