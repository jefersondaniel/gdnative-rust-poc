use std::{collections::HashMap, sync::Arc};

use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};
use gdnative::core_types::Rid;
use gdnative::api::VisualServer;

use crate::{systems::visual_server::{enumerations::VisualServerStage, root_node::RootNode, transform::Transform, texture::Texture}};

use super::{common::Text, font_loader::FontLoader, text_renderer::render_text, vector_font::VectorFontCacheKey};

struct RidMap {
    pub canvas_items: HashMap<u32, Rid>,
}

pub struct VectorFontCache(HashMap<VectorFontCacheKey, Arc<Texture>>);

impl Default for VectorFontCache {
    fn default() -> Self { Self(HashMap::new()) }
}

#[derive(Default, Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub transform: Transform,
    pub vector_font_cache: VectorFontCache,
}

#[derive(Default)]
pub struct TextPlugin;

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut rid_map: ResMut<RidMap>,
    mut query: Query<
        (Entity, &Text, &mut VectorFontCache, &Transform),
        Changed<Text>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, text, mut vector_font_cache, transform) in query.iter_mut() {
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
        render_text(visual_server, rid, &text, &mut vector_font_cache.0);
        visual_server.canvas_item_set_transform(rid, transform.into());
    }
}

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
