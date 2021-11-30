use bevy_app::App;
use bevy_core::CorePlugin;
use gdnative::{api::DynamicFont, prelude::*};

use crate::{drawing::sprite_system::SpriteSystem, io::file_system::FileSystem, systems::{debug::DebugPlugin, menu::menu_plugin::MenuPlugin, visual_server::{root_node::RootNode, visual_server_plugin::VisualServerPlugin}}};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    app: App,
    #[property]
    store: Dictionary,
}

#[methods]
impl Game {
    pub fn new(owner: TRef<Node2D>) -> Self {
        let store = Dictionary::new_shared();
        let root_node = RootNode::new(&owner, store.new_ref());

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
            ),
            store: store.new_ref(),
        }
    }

    #[export]
    pub fn _process(&mut self, _owner: TRef<Node2D>, _delta: Variant) {
        self.app.update();
    }
}
