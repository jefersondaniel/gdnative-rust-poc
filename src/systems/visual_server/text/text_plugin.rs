use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};
use gdnative::core_types::{Transform2D};
use gdnative::api::{VisualServer};

use crate::systems::visual_server::canvas_item::{Visible, ClipRect, setup_canvas_item, CanvasItemState, CanvasItem, ZIndex, BackBufferCopy};
use crate::systems::visual_server::material::Material;
use crate::{systems::visual_server::{enumerations::VisualServerStage, root_node::RootNode, texture::Texture}};

use super::{common::Text, text_renderer::render_text, vector_font::VectorFontCacheKey};

pub struct VectorFontCache(HashMap<VectorFontCacheKey, Arc<Texture>>);

impl Default for VectorFontCache {
    fn default() -> Self { Self(HashMap::new()) }
}

#[derive(Default, Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub vector_font_cache: VectorFontCache,
    pub canvas_item: CanvasItem,
    pub visible: Visible,
    pub back_buffer_copy: BackBufferCopy,
    pub transform: Transform2D,
    pub clip_rect: Option<ClipRect>,
    pub material: Option<Arc<RwLock<Material>>>,
    pub z_index: ZIndex,
}

#[derive(Default)]
pub struct TextPlugin;

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut canvas_item_state: ResMut<CanvasItemState>,
    mut query: Query<
        (Entity, &Text, &mut VectorFontCache, &mut CanvasItem, &Transform2D, &Visible, &BackBufferCopy, &Option<Arc<RwLock<Material>>>, &Option<ClipRect>),
        Changed<Text>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        text,
        mut vector_font_cache,
        mut canvas_item,
        transform,
        visible,
        back_buffer_copy,
        material,
        clip_rect
    ) in query.iter_mut() {
        setup_canvas_item(
            &entity,
            visual_server,
            &root_node,
            &mut canvas_item_state,
            &mut canvas_item,
            transform,
            visible,
            back_buffer_copy,
            material,
            clip_rect,
        );

        visual_server.canvas_item_set_parent(canvas_item.rid, root_node.canvas_item_rid);
        render_text(visual_server, canvas_item.rid, &text, &mut vector_font_cache.0);
        visual_server.canvas_item_set_transform(canvas_item.rid, *transform);
    }
}

impl Plugin for TextPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .add_system_to_stage(VisualServerStage::Update, update_canvas_item.system());
    }
}
