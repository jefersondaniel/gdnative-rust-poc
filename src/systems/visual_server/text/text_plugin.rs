use std::{collections::HashMap};

use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};
use gdnative::core_types::Rid;
use gdnative::api::VisualServer;

use crate::{systems::visual_server::{enumerations::VisualServerStage, root_node::RootNode, transform::Transform}};

use super::{common::Text, font_loader::FontLoader, text_renderer::render_text};

struct RidMap {
    pub canvas_items: HashMap<u32, Rid>,
}

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut rid_map: ResMut<RidMap>,
    mut query: Query<
        (Entity, &mut Text, &Transform),
        Changed<Text>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (entity, mut text, transform) in query.iter_mut() {
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
        render_text(visual_server, rid, &mut text);
        visual_server.canvas_item_set_transform(rid, transform.into());
    }
}

#[derive(Default, Bundle)]
pub struct TextBundle {
    pub text: Text,
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
