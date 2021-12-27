use std::{sync::{Arc, RwLock}, collections::HashMap};

use gdnative::{api::VisualServer, core_types::{Rid, Variant}};

use super::{shader::Shader, texture::Texture};

pub struct Material {
    pub rid: Rid,
    pub shader: Arc<Shader>,
    pub texture_parameters: HashMap<String, Arc<Texture>>,
}

impl Material {
    pub fn allocate(shader: Arc<Shader>) -> Arc<RwLock<Self>> {
        let singleton = unsafe { VisualServer::godot_singleton() };
        let rid = singleton.material_create();

        singleton.material_set_shader(rid, shader.rid);

        Arc::new(RwLock::new(Self {
            rid,
            shader,
            texture_parameters: HashMap::new()
        }))
    }

    pub fn set_shader_param(&mut self, parameter: &str, value: Variant) {
        let singleton = unsafe { VisualServer::godot_singleton() };
        singleton.material_set_param(self.rid, parameter, value);
    }

    pub fn set_shader_texture(&mut self, parameter: &str, texture: Arc<Texture>) {
        self.set_shader_param(parameter, texture.to_variant());
        self.texture_parameters.insert(parameter.to_string(), texture);
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
