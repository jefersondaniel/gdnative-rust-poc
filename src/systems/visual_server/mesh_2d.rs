use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};
use gdnative::NewRef;
use gdnative::api::visual_server::PrimitiveType;
use gdnative::core_types::{Transform2D};
use std::collections::{HashSet,HashMap};
use std::sync::RwLock;
use std::sync::Arc;
use gdnative::{api::VisualServer, core_types::{Color, VariantArray, Rid}};

use super::canvas_item::{ClipRect, CanvasItem, ZIndex, BackBufferCopy, GlobalTransform};
use super::{root_node::RootNode, texture::Texture};

use crate::systems::visual_server::material::Material;
use crate::systems::visual_server::canvas_item::Visible;
use crate::systems::visual_server::enumerations::{VisualServerStage};

struct RidMap {
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
    pub canvas_item: CanvasItem,
    pub mesh: Mesh2d,
    pub texture: Arc<Texture>,
    pub visible: Visible,
    pub transform: Transform2D,
    pub global_transform: GlobalTransform,
    pub material: Option<Arc<RwLock<Material>>>,
    pub clip_rect: ClipRect,
    pub back_buffer_copy: BackBufferCopy,
    pub z_index: ZIndex,
}

impl Default for Mesh2dBundle {
    fn default() -> Self {
        Mesh2dBundle {
            canvas_item: CanvasItem::default(),
            mesh: Mesh2d::default(),
            texture: Arc::new(Texture::invalid()),
            visible: Visible::default(),
            back_buffer_copy: BackBufferCopy::default(),
            transform: Transform2D::default(),
            global_transform: GlobalTransform::default(),
            z_index: ZIndex::default(),
            material: None,
            clip_rect: ClipRect::default()
        }
    }
}

fn update_meshes(
    mut rid_map: ResMut<RidMap>,
    query: Query<
        (Entity, &CanvasItem, &Mesh2d, &Arc<Texture>),
        // NOTE: Change detection here must be in sync with canvas_item.rs
        Or<(Changed<Mesh2d>, Changed<Arc<Texture>>, Changed<CanvasItem>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        canvas_item,
        mesh_2d,
        texture
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

        let xform = Transform2D::translation(0.0, 0.0);
        let modulate = Color::rgba(1.0, 1.0, 1.0, 1.0);

        visual_server.canvas_item_add_mesh(
            canvas_item.rid,
            mesh_rid,
            xform.into(),
            modulate,
            texture.rid,
            Rid::new()
        );
    }
}

fn remove_meshes(
    mut rid_map: ResMut<RidMap>,
    removals: RemovedComponents<Mesh2d>,
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };
    let mut affected_entity_ids = HashSet::new();

    for entity in removals.iter() {
        if let Some(rid) = rid_map.meshes.get(&entity.id()) {
            affected_entity_ids.insert(entity.id());
            visual_server.free_rid(*rid);
        }
    }

    for id in affected_entity_ids.iter() {
        rid_map.meshes.remove(id);
    }
}

#[derive(Default)]
pub struct Mesh2dPlugin;

impl Plugin for Mesh2dPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .insert_resource(RidMap { meshes: HashMap::new() })
            .add_system_to_stage(VisualServerStage::Remove, remove_meshes.system())
            .add_system_to_stage(VisualServerStage::Update, update_meshes.system());
    }
}
