use gdnative::api::visual_server::TextureFlags;
use gdnative::core_types::{Point2, Size2};

use crate::core::error::DataError;
use crate::systems::visual_server::texture::Texture;

use super::image::{Palette, RawImage};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct SffPal {
    pub pal: Arc<Palette>,
    pub itemno: i32,
    pub groupno: i32,
    pub is_used: bool,
    pub usedby: i32,
    pub reserved: i32,
}

#[derive(Clone)]
pub struct MutableSffData {
    pub image: Rc<RefCell<RawImage>>,
    pub groupno: i16,
    pub imageno: i16,
    pub x: i16,
    pub y: i16,
    pub palindex: i16,
    pub linked: i16,
}

impl MutableSffData {
    pub fn to_sff_data(&self) -> SffData {
        SffData {
            image: Arc::new(self.image.borrow().clone()),
            groupno: self.groupno,
            imageno: self.imageno,
            x: self.x,
            y: self.y,
            palindex: self.palindex,
            linked: self.linked,
        }
    }
}

#[derive(Clone)]
pub struct SffData {
    pub image: Arc<RawImage>,
    pub groupno: i16,
    pub imageno: i16,
    pub x: i16,
    pub y: i16,
    pub palindex: i16,
    pub linked: i16,
}

#[derive(Clone)]
pub struct SffMetadata {
    pub verlo3: u8,
    pub verlo2: u8,
    pub verlo1: u8,
    pub verhi: u8,
}

impl SffData {
    pub fn create_texture(
        &self,
        preferred_palette: Option<Arc<Palette>>,
        flags: TextureFlags
    ) -> Result<Arc<Texture>, DataError> {
        let raw_image = self.image.clone();

        let palette = match preferred_palette {
            Some(palette) => {
                if palette.is_empty() || self.palindex != 0 {
                    self.image.color_table.clone()
                } else {
                    palette.clone()
                }
            }
            None => self.image.color_table.clone()
        };

        let image = raw_image.create_image_with_palette(&palette);

        Ok(Texture::allocate(
            image,
            flags
        ))
    }

    pub fn create_monochromatic_texture(&self, flags: TextureFlags) -> Arc<Texture> {
        self.image.create_monochromatic_texture(flags)
    }

    pub fn create_palette_texture(&self, palette: Option<Arc<Palette>>) -> Arc<Texture> {
        match palette {
            Some(palette) => {
                if palette.is_empty() || self.palindex == 0 {
                    self.image.color_table.create_texture()
                } else {
                    palette.create_texture()
                }
            },
            None => self.image.color_table.create_texture()
        }
    }

    pub fn offset(&self) -> Point2 {
        return Point2::new(-self.x as f32, -self.y as f32)
    }
}
