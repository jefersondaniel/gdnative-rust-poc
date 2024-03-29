use std::sync::Arc;

use gdnative::{Ref, api::{Image, VisualServer, visual_server::TextureFlags}, core_types::{Rid, Size2, Variant}, prelude::Unique};

pub struct Texture {
    pub rid: Rid,
    pub size: Size2,
}

impl Texture {
    pub fn allocate(image: Ref<Image, Unique>, flags: TextureFlags) -> Arc<Self> {
        let size = Size2::new(
            image.get_width() as f32,
            image.get_height() as f32
        );
        let singleton = unsafe { VisualServer::godot_singleton() };
        let rid = singleton.texture_create_from_image(image, i64::from(flags));

        Arc::new(Texture {
            rid: rid,
            size
        })
    }

    pub fn invalid() -> Self {
        Texture {
            rid: Rid::new(),
            size: Size2::new(0.0, 0.0)
        }
    }

    pub fn to_variant(&self) -> Variant {
        Variant::from_rid(&self.rid)
    }
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        if self.rid.is_valid() {
            let singleton = unsafe { VisualServer::godot_singleton() };
            singleton.free_rid(self.rid);
        }
    }
}
