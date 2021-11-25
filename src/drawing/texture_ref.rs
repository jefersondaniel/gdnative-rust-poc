use std::rc::Rc;

use gdnative::{api::{VisualServer, visual_server::TextureFlags}, core_types::{Rid, Size2, Vector2}};

use crate::core::error::DataError;

use super::sff::{image::Palette, sff_common::SffData};

pub struct TextureRef {
    pub rid: Rid,
    pub offset: Vector2,
    pub size: Size2,
}

impl TextureRef {
    pub fn allocate(
        sff_data: SffData,
        palette: Option<Rc<Palette>>,
        flags: TextureFlags,
    ) -> Result<Rc<Self>, DataError> {
        let mut raw_image = sff_data.image.borrow().clone();

        if let Some(palette_rc) = palette {
            if !palette_rc.is_empty() {
                if sff_data.palindex == 0 {
                    raw_image.color_table = Rc::clone(&palette_rc);
                }
            }
        }

        let singleton = unsafe { VisualServer::godot_singleton() };
        let image = raw_image.create_image();
        let offset = Vector2::new(
            sff_data.x as f32,
            sff_data.y as f32
        );
        let size = Size2::new(
            image.get_width() as f32,
            image.get_height() as f32
        );
        let rid = singleton.texture_create_from_image(image, i64::from(flags));

        Ok(Rc::new(TextureRef {
            rid: rid,
            offset,
            size
        }))
    }
}

impl Drop for TextureRef {
    fn drop(&mut self) {
        let singleton = unsafe { VisualServer::godot_singleton() };

        singleton.free_rid(self.rid);
    }
}
