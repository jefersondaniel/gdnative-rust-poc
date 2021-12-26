use std::{collections::HashMap, sync::Arc};

use crate::{core::blending::Blending, systems::visual_server::shader::Shader};

pub struct ShaderManager {
    blending_shader_map: HashMap<Blending, Arc<Shader>>,
}

impl ShaderManager {
    pub fn get_blending_shader(&mut self, blending: &Blending) -> Arc<Shader> {
        let shader_option = self.blending_shader_map.get(blending);

        if let Some(shader) = shader_option {
            return shader.clone();
        }

        let code = "";
        let shader = Shader::allocate(code);
        self.blending_shader_map.insert(*blending, shader.clone());

        return shader.clone();
    }
}

