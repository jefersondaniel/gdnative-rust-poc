use gdnative::core_types::{Rect2, Vector2};

use crate::core::enumerations::{ClsnType, Facing};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Clsn {
    pub clsn_type: ClsnType,
    pub rect: Rect2,
}

impl Clsn {
    pub fn new(clsn_type: ClsnType, rect: Rect2) -> Self {
        Clsn {
            clsn_type: clsn_type,
            rect: rect,
        }
    }
}
