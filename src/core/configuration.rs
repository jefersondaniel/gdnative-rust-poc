use std::sync::Arc;

use gdnative::core_types::{Size2};

use crate::systems::visual_server::shader::Shader;

pub struct Configuration {
    pub screen_size: Size2,
    pub sprite_shader: Arc<Shader>,
}
