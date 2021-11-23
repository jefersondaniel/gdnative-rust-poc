use bevy_ecs::prelude::*;
use gdnative::godot_error;

use crate::core::error::DataError;

pub fn handle_error(In(result): In<Result<(), DataError>>) {
    if let Err(e) = result {
        godot_error!("{}", e);
    }
}
