use bevy_app::{AppBuilder, CoreStage, Plugin};
use bevy_ecs::prelude::*;

use super::{enumerations::VisualServerStage, sprite::SpritePlugin};


#[derive(Default)]
pub struct VisualServerPlugin;

impl Plugin for VisualServerPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .add_stage_after(CoreStage::Update, VisualServerStage::Update, SystemStage::single_threaded())
            .add_plugin(SpritePlugin::default());
    }
}