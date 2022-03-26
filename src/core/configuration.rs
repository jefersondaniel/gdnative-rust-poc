use std::sync::Arc;

use gdnative::core_types::{Size2, Point2};

use crate::systems::visual_server::shader::Shader;

pub struct Configuration {
    pub screen_size: Size2,
    pub sprite_shader: Arc<Shader>,
}

pub trait ScaleForScreen {
    fn scale_for_screen(&self, configuration: &Configuration, localcoord: Size2) -> Self;
}

impl ScaleForScreen for Size2 {
    fn scale_for_screen(&self, configuration: &Configuration, localcoord: Size2) -> Self {
        if self.width == 0.0 && self.height == 0.0 {
            return self.clone();
        }

        let factor = configuration.screen_size.width / localcoord.width;

        return Self::new(
            self.width * factor,
            self.height * factor
        );
    }
}

impl ScaleForScreen for Point2 {
    fn scale_for_screen(&self, configuration: &Configuration, localcoord: Size2) -> Self {
        if self.x == 0.0 && self.y == 0.0 {
            return self.clone();
        }

        let factor = configuration.screen_size.width / localcoord.width;

        return Self::new(
            self.x * factor,
            self.y * factor
        );
    }
}
