use std::sync::Arc;

use gdnative::{api::VisualServer, core_types::Rid};

pub struct Shader {
    pub rid: Rid,
}

impl Shader {
    pub fn allocate(code: &str) -> Arc<Self> {
        let singleton = unsafe { VisualServer::godot_singleton() };
        let rid = singleton.shader_create();

        singleton.shader_set_code(rid, code);

        Arc::new(Self {
            rid,
        })
    }
}

impl Drop for Shader {
    #[inline]
    fn drop(&mut self) {
        if self.rid.is_valid() {
            let singleton = unsafe { VisualServer::godot_singleton() };
            singleton.free_rid(self.rid);
        }
    }
}
