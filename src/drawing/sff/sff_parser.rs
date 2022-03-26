use std::sync::Arc;

use crate::core::error::DataError;

use super::image::Palette;
use super::sff_common::{SffData, SffMetadata};
use super::sffv1;
use super::sffv2;

pub fn read_metadata(path: &str) -> Result<SffMetadata, DataError> {
    let result_v2 = sffv2::read_metadata(&path);

    if result_v2.is_err() {
        return sffv1::read_metadata(&path);
    }

    result_v2
}

pub fn read_palette(path: &str) -> Result<Arc<Palette>, DataError> {
    sffv1::read_palette(&path)
}

pub fn read_palettes(path: &str) -> Result<Vec<Arc<Palette>>, DataError> {
    sffv2::read_palettes(&path)
}

pub fn read_images(path: &str, groups: &[i16]) -> Result<Vec<SffData>, DataError> {
    let result_v2 = sffv2::read_images(&path, &groups);

    if result_v2.is_err() {
        return sffv1::read_images(&path, &groups);
    }

    result_v2
}
