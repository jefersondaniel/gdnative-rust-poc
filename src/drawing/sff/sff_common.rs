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
    pub pal: Rc<Palette>,
    pub itemno: i32,
    pub groupno: i32,
    pub is_used: bool,
    pub usedby: i32,
    pub reserved: i32,
}

#[derive(Clone)]
pub struct SffData {
    pub image: Rc<RefCell<RawImage>>,
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
        palette: Option<Rc<Palette>>,
        flags: TextureFlags
    ) -> Result<Arc<Texture>, DataError> {
        let mut raw_image = self.image.borrow().clone();

        if let Some(palette_rc) = palette {
            if !palette_rc.is_empty() {
                if self.palindex == 0 {
                    raw_image.color_table = Rc::clone(&palette_rc);
                }
            }
        }

        let image = raw_image.create_image();

        Ok(Texture::allocate(
            image,
            flags
        ))
    }

    pub fn create_monochromatic_texture(&self, flags: TextureFlags) -> Arc<Texture> {
        self.image.borrow().create_monochromatic_texture(flags)
    }

    pub fn create_palette_texture(&self, palette: Option<Rc<Palette>>) -> Arc<Texture> {
        let mut raw_image = self.image.borrow().clone();

        if let Some(palette_rc) = palette {
            if !palette_rc.is_empty() {
                if self.palindex == 0 {
                    raw_image.color_table = Rc::clone(&palette_rc);
                }
            }
        }

        raw_image.create_palette_texture()
    }
}
