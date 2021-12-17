use bevy_app::{AppBuilder, CoreStage, Plugin};
use bevy_ecs::prelude::*;

use super::{enumerations::VisualServerStage, sprite::SpritePlugin, text::text_plugin::TextPlugin, mesh_2d::Mesh2dPlugin, canvas_item::CanvasItemPlugin};


#[derive(Default)]
pub struct VisualServerPlugin;

impl Plugin for VisualServerPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .add_stage_after(CoreStage::Update, VisualServerStage::Remove, SystemStage::single_threaded())
            .add_stage_after(VisualServerStage::Remove, VisualServerStage::Update, SystemStage::single_threaded())
            .add_stage_after(VisualServerStage::Update, VisualServerStage::Transform, SystemStage::single_threaded())
            .add_plugin(CanvasItemPlugin::default())
            .add_plugin(SpritePlugin::default())
            .add_plugin(TextPlugin::default())
            .add_plugin(Mesh2dPlugin::default());
    }
}
