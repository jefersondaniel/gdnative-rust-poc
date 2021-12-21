use bevy_app::{AppBuilder, Plugin};

use crate::systems::visual_server::root_node::RootNode;

use super::audio::Audio;

#[derive(Default)]
pub struct AudioServerPlugin;

impl Plugin for AudioServerPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        let root_node = {
            let world = &builder.app.world;

            world.get_resource::<RootNode>().expect("Root node not found")
        };

        builder.insert_resource(Audio::new(root_node.node.clone()));
    }
}
