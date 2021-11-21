use crate::core::error::DataError;

use super::image::Palette;
use super::sff_common::{SffData, SffMetadata};
use super::sffv1;
use super::sffv2;
use std::rc::Rc;

pub struct SffParser {

}

impl SffParser {
    pub fn new() -> Self {
        SffParser {
        }
    }

    pub fn read_metadata(&mut self, path: &str) -> Result<SffMetadata, DataError> {
        let result_v2 = sffv2::read_metadata(&path);

        if result_v2.is_err() {
            return sffv1::read_metadata(&path);
        }

        result_v2
    }

    pub fn read_palette(&mut self, path: &str) -> Result<Rc<Palette>, DataError> {
        sffv1::read_palette(&path)
    }

    pub fn read_palettes(&mut self, path: &str) -> Result<Vec<Rc<Palette>>, DataError> {
        sffv2::read_palettes(&path)
    }

    pub fn read_images(&mut self, path: &str, groups: Vec<i16>) -> Result<Vec<SffData>, DataError> {
        let result_v2 = sffv2::read_images(&path, &groups);

        if result_v2.is_err() {
            return sffv1::read_images(&path, &groups);
        }

        result_v2
    }
}
