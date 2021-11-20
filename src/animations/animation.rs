use gdnative::core_types::Vector2;
use crate::core::{blending::Blending, enumerations::SpriteEffects, sprite_id::SpriteId};

#[derive(Copy, Clone, PartialEq)]
pub struct AnimationElement {
    pub id: usize,
    pub gameticks: i32,
    pub sprite_id: SpriteId,
    pub offset: Vector2,
    pub flip: SpriteEffects,
    pub blending: Blending,
    pub start_tick: i32,
}

impl AnimationElement {
    pub fn new(
        id: usize,
        gameticks: i32,
        sprite_id: SpriteId,
        offset: Vector2,
        flip: SpriteEffects,
        blending: Blending,
        start_tick: i32,
    ) -> Self {
        AnimationElement {
            id: id,
            gameticks: gameticks,
            sprite_id: sprite_id,
            offset: offset,
            flip: flip,
            blending: blending,
            start_tick: start_tick,
        }
    }
}

#[derive(Clone)]
pub struct Animation {
    pub number: usize,
    pub loopstart: usize,
    pub elements: Vec<AnimationElement>,
    pub totaltime: i32,
}

fn calculate_time(elements: &Vec<AnimationElement>) -> i32 {
    let mut time = 0;

    for element in elements.iter() {
        if element.gameticks == -1 {
            return -1;
        }

        time += element.gameticks;
    }

    time
}

impl Animation {
    pub fn new(
        number: usize,
        loopstart: usize,
        elements: Vec<AnimationElement>,
    ) -> Self {
        let totaltime = calculate_time(&elements);

        Animation {
            number: number,
            loopstart: loopstart,
            elements: elements,
            totaltime: totaltime,
        }
    }
}
