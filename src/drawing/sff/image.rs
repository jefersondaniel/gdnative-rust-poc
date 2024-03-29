use gdnative::Ref;
use gdnative::api::image::Image;
use gdnative::api::visual_server::TextureFlags;
use gdnative::core_types::ByteArray;
use gdnative::prelude::Unique;
use std::sync::Arc;

use crate::systems::visual_server::texture::Texture;

#[derive(Copy, Clone)]
pub struct RawColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RawColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RawColor {
        RawColor { r, g, b, a }
    }

    pub fn empty() -> RawColor {
        RawColor {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn equal(&self, other: &RawColor) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

#[derive(Clone)]
pub struct Palette {
    pub colors: Vec<RawColor>,
}

impl Palette {
    pub fn new(num_colors: usize) -> Palette {
        let colors = vec![RawColor::empty(); num_colors];
        Palette { colors }
    }

    pub fn from_colors(colors: Vec<RawColor>) -> Palette {
        Palette { colors }
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    pub fn equal(&self, other: &Palette) -> bool {
        if self.colors.len() != other.colors.len() {
            return false;
        }

        for (i, color) in self.colors.iter().enumerate() {
            if !color.equal(&other.colors[i]) {
                return false;
            }
        }

        true
    }

    pub fn create_texture(&self) -> Arc<Texture> {
        let width = self.colors.len();
        let mut my_byte_array: Vec<u8> = Vec::with_capacity(width * 4);

        for color in self.colors.iter() {
            my_byte_array.push(color.r);
            my_byte_array.push(color.g);
            my_byte_array.push(color.b);
            my_byte_array.push(color.a);
        }

        let dest = ByteArray::from_slice(my_byte_array.as_slice());
        let image = Image::new();

        image.create_from_data(
            width as i64,
            1 as i64,
            false,
            Image::FORMAT_RGBA8,
            dest,
        );

        Texture::allocate(image, TextureFlags(0))
    }
}

#[derive(Clone)]
pub struct RawImage {
    pub w: usize,
    pub h: usize,
    pub pixels: Arc<Vec<u8>>,
    pub color_table: Arc<Palette>,
}

impl RawImage {
    pub fn empty() -> RawImage {
        let pixels = Arc::new(Vec::new());
        let color_table = Arc::new(Palette::new(0));

        RawImage {
            w: 0,
            h: 0,
            pixels,
            color_table,
        }
    }

    pub fn create_image_with_palette(&self, palette: &Palette) -> Ref<Image, Unique> {
        let mut my_byte_array: Vec<u8> = Vec::with_capacity(self.w * self.h * 4);

        for &pixel in self.pixels.iter() {
            let color = &palette.colors[pixel as usize];
            my_byte_array.push(color.r);
            my_byte_array.push(color.g);
            my_byte_array.push(color.b);
            my_byte_array.push(color.a);
        }

        let dest = ByteArray::from_slice(my_byte_array.as_slice());

        let image = Image::new();
        image.create_from_data(
            self.w as i64,
            self.h as i64,
            false,
            Image::FORMAT_RGBA8,
            dest,
        );
        image
    }

    pub fn create_image(&self) -> Ref<Image, Unique> {
        self.create_image_with_palette(&self.color_table)
    }

    pub fn create_monochromatic_texture(&self, flags: TextureFlags) -> Arc<Texture> {
        let dest = ByteArray::from_slice(self.pixels.as_slice());

        let image = Image::new();

        image.create_from_data(
            self.w as i64,
            self.h as i64,
            false,
            Image::FORMAT_R8,
            dest,
        );

        Texture::allocate(image, flags)
    }

    pub fn create_palette_texture(&self) -> Arc<Texture> {
        let width = self.color_table.colors.len();
        let mut my_byte_array: Vec<u8> = Vec::with_capacity(width * 4);

        for color in self.color_table.colors.iter() {
            my_byte_array.push(color.r);
            my_byte_array.push(color.g);
            my_byte_array.push(color.b);
            my_byte_array.push(color.a);
        }

        let dest = ByteArray::from_slice(my_byte_array.as_slice());
        let image = Image::new();

        image.create_from_data(
            width as i64,
            1 as i64,
            false,
            Image::FORMAT_RGBA8,
            dest,
        );

        Texture::allocate(image, TextureFlags(0))
    }
}
