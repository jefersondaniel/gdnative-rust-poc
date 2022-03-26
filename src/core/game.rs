use std::time::SystemTime;

use bevy_app::App;
use bevy_core::CorePlugin;
use bevy_transform::TransformPlugin;
use gdnative::{prelude::{NativeClass,Node2D,TRef,methods,FromVariant,Variant}};

use crate::{drawing::sprite_system::SpriteSystem, systems::{debug::DebugPlugin, menu::menu_plugin::MenuPlugin, visual_server::{root_node::RootNode, time::DeltaTime, visual_server_plugin::VisualServerPlugin}, input::Input, audio_server::audio_server_plugin::AudioServerPlugin, backgrounds::background_plugin::BackgroundPlugin}, profiles::profile_loader::ProfileLoader};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    app: App,
}

#[methods]
impl Game {
    pub fn new(owner: TRef<Node2D>) -> Self {
        let root_node = RootNode::new(&owner);
        let input = Input::default();

        Game {
            app: std::mem::take(
                &mut App::build()
                .insert_resource(root_node)
                .insert_resource(input)
                .insert_resource(DeltaTime::default())
                .insert_resource(SpriteSystem::new())
                .add_plugin(CorePlugin::default())
                .add_plugin(TransformPlugin::default())
                .add_plugin(VisualServerPlugin::default())
                .add_plugin(AudioServerPlugin::default())
                // .add_plugin(DebugPlugin::default())
                .add_plugin(BackgroundPlugin::default())
                .add_plugin(MenuPlugin::default())
                .app
            )
        }
    }

    #[export]
    pub fn _process(&mut self, _owner: TRef<Node2D>, _delta: Variant) {
        {
            let mut delta_time = self.app.world.get_resource_mut::<DeltaTime>().unwrap();
            delta_time.0 = f64::from_variant(&_delta).unwrap();
        }

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
            Err(_) => {}
        }
    }
}
