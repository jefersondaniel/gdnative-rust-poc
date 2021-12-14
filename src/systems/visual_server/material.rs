use std::sync::{Arc, RwLock};

use gdnative::{api::VisualServer, core_types::{Rid, Variant}};

use super::shader::Shader;

pub struct Material {
    pub rid: Rid,
    pub shader: Arc<Shader>,
}

impl Material {
    pub fn allocate(shader: Arc<Shader>) -> Arc<RwLock<Self>> {
        let singleton = unsafe { VisualServer::godot_singleton() };
        let rid = singleton.material_create();

        singleton.material_set_shader(rid, shader.rid);

        Arc::new(RwLock::new(Self {
            rid,
            shader
        }))
    }

    pub fn set_shader_param(&mut self, parameter: String, value: Variant) {
        let singleton = unsafe { VisualServer::godot_singleton() };
        singleton.material_set_param(self.rid, parameter, value);
    }
}

impl Drop for Material {
    #[inline]
    fn drop(&mut self) {
        if self.rid.is_valid() {
            let singleton = unsafe { VisualServer::godot_singleton() };
            singleton.free_rid(self.rid);
        }
    }
}
