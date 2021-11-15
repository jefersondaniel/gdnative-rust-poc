use gdnative::core_types::Vector2;
use crate::core::{blending::Blending, enumerations::SpriteEffects, sprite_id::SpriteId};

#[derive(Copy, Clone, PartialEq)]
pub struct AnimationElement {
    id: i32,
    gameticks: i32,
    sprite_id: SpriteId,
    offset: Vector2,
    flip: SpriteEffects,
    blending: Blending,
    start_tick: i32,
}

#[derive(Clone)]
pub struct Animation {
    pub number: i32,
    pub loopstart: i32,
    pub totaltime: i32,
    pub elements: Vec<AnimationElement>,
}
