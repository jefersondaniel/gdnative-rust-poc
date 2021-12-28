use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

use bevy_ecs::prelude::*;
use bevy_app::{AppBuilder, Plugin};
use gdnative::core_types::{Transform2D};
use gdnative::api::{VisualServer};

use crate::systems::visual_server::canvas_item::{Visible, ClipRect, CanvasItemState, CanvasItem, ZIndex, BackBufferCopy, GlobalTransform};
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
    pub global_transform: GlobalTransform,
    pub clip_rect: ClipRect,
    pub material: Option<Arc<RwLock<Material>>>,
    pub z_index: ZIndex,
}

#[derive(Default)]
pub struct TextPlugin;

fn update_text(
    mut query: Query<
        (&CanvasItem, &Text, &mut VectorFontCache),
        // NOTE: Change detection here must be in sync with canvas_item.rs
        Changed<Text>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (canvas_item, text, mut vector_font_cache) in query.iter_mut() {
        render_text(visual_server, canvas_item.rid, &text, &mut vector_font_cache.0);
    }
}

impl Plugin for TextPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .add_system_to_stage(VisualServerStage::Update, update_text.system());
    }
}
