use std::time::SystemTime;

use bevy_app::App;
use bevy_core::CorePlugin;
use gdnative::{api::DynamicFont, prelude::*};

use crate::{drawing::sprite_system::SpriteSystem, io::file_system::FileSystem, systems::{debug::DebugPlugin, menu::menu_plugin::MenuPlugin, visual_server::{root_node::RootNode, visual_server_plugin::VisualServerPlugin}}};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    app: App,
}

#[methods]
impl Game {
    pub fn new(owner: TRef<Node2D>) -> Self {
        let root_node = RootNode::new(&owner);

        Game {
            app: std::mem::take(
                &mut App::build()
                .insert_resource(root_node)
                .insert_resource(FileSystem::new())
                .insert_resource(SpriteSystem::new())
                .add_plugin(CorePlugin::default())
                .add_plugin(VisualServerPlugin::default())
                .add_plugin(DebugPlugin::default())
                // .add_plugin(MenuPlugin::default())
                .app
            )
        }
    }

    #[export]
    pub fn _process(&mut self, _owner: TRef<Node2D>, _delta: Variant) {
        let now = SystemTime::now();
        self.app.update();
        match now.elapsed() {
            Ok(elapsed) => {
                let limit = 16u128;
                let frametime = elapsed.as_millis();
                if frametime > limit {
                    gdnative::godot_warn!("High Frametime: {}ms", frametime);
                }
            }
            Err(e) => {}
        }
    }
}
