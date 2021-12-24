use std::sync::{Arc, RwLock};

use bevy_app::{AppBuilder, Plugin};
use bevy_ecs::prelude::*;
use gdnative::core_types::Transform2D;
use gdnative::{api::VisualServer, core_types::{Color, Point2, Rect2, Rid, Size2}};

use crate::systems::visual_server::enumerations::{VisualServerStage};
use crate::systems::visual_server::canvas_item::Visible;

use super::canvas_item::{CanvasItem, ZIndex};
use super::canvas_item::CanvasItemState;
use super::canvas_item::ClipRect;
use super::canvas_item::setup_canvas_item;
use super::{root_node::RootNode, texture::Texture, material::Material};

#[derive(Clone, Default)]
pub struct Sprite {
    pub size: Size2,
    pub rect: Option<Rect2>,
    pub offset: Point2,
    pub flip_v: bool,
    pub flip_h: bool,
}

#[derive(Bundle)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub canvas_item: CanvasItem,
    pub texture: Arc<Texture>,
    pub visible: Visible,
    pub transform: Transform2D,
    pub clip_rect: Option<ClipRect>,
    pub material: Option<Arc<RwLock<Material>>>,
    pub z_index: ZIndex,
}

impl Default for SpriteBundle {
    fn default() -> Self {
        SpriteBundle {
            sprite: Sprite::default(),
            texture: Arc::new(Texture::invalid()),
            visible: Visible::default(),
            transform: Transform2D::default(),
            canvas_item: CanvasItem::default(),
            z_index: ZIndex::default(),
            clip_rect: None,
            material: None,
        }
    }
}

fn update_canvas_item(
    root_node: Res<RootNode>,
    mut canvas_item_state: ResMut<CanvasItemState>,
    mut query: Query<
        (Entity, &Sprite, &mut CanvasItem, &Arc<Texture>, &Transform2D, &Visible, &Option<Arc<RwLock<Material>>>, &Option<ClipRect>),
        Or<(Changed<Sprite>, Changed<Arc<Texture>>, Changed<Option<ClipRect>>)>
    >
) {
    let visual_server = unsafe { VisualServer::godot_singleton() };

    for (
        entity,
        sprite,
        mut canvas_item,
        texture,
        transform,
        visible,
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
            material,
            clip_rect,
        );

        let mut dst_rect = Rect2::new(
            sprite.offset,
            sprite.size
        );

        let src_rect = match sprite.rect {
            Some(rect) => rect,
            None => Rect2::new(Point2::default(), texture.size)
        };

        if sprite.flip_v {
            dst_rect.size.height = -dst_rect.size.height;
        }

        if sprite.flip_h {
            dst_rect.size.width = -dst_rect.size.width;
        }

        visual_server.canvas_item_add_texture_rect_region(
            canvas_item.rid,
            dst_rect,
            texture.rid,
            src_rect,
            Color::rgba(1.0, 1.0, 1.0, 1.0),
            false,
            Rid::new(),
            false
        );
    }
}

#[derive(Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .add_system_to_stage(VisualServerStage::Update, update_canvas_item.system());
    }
}
