use crate::systems::visual_server::root_node::RootNode;
use bevy_ecs::prelude::Res;
use gdnative::{api::VisualServer, core_types::{Color, Point2, Rect2, Rid, Size2}};
use std::collections::HashMap;

pub struct Visible {
    pub is_visible: bool,
}

impl Default for Visible {
    fn default() -> Self { Visible { is_visible: true } }
}

pub struct ClipRect(Rect2);

pub fn build_canvas_item(visual_server: &VisualServer, entity_id: u32, rid_map: &mut HashMap<u32, Rid>) -> Rid {
    match rid_map.get(&entity_id) {
        Some(value) => {
            visual_server.canvas_item_clear(*value);
            *value
        },
        None => {
            let rid = visual_server.canvas_item_create();
            rid_map.insert(entity_id, rid);
            rid
        }
    }
}
