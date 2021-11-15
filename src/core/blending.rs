use gdnative::api::animation_node_blend_space_2d::BlendMode;

use super::enumerations::BlendType;

#[derive(Copy, Clone, PartialEq)]
pub struct Blending {
    blend_type: BlendType,
    source: u8,
    destination: u8,
}

impl Blending {
    pub fn new(
        blend_type: BlendType,
        source: u8,
        destination: u8
    ) -> Self {
        Blending {
            blend_type: blend_type,
            source: if blend_type != BlendType::None { source } else { 0 },
            destination: if blend_type != BlendType::None { destination } else { 0 }
        }
    }
}
