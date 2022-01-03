use bevy_ecs::prelude::Entity;

use crate::backgrounds::background_group::BackgroundGroup;

pub struct BackgroundGroupEvent {
    pub layer: Entity,
    pub background_group: BackgroundGroup,
}
