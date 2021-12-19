use gdnative::api::Input as GodotInput;

#[derive(Default)]
pub struct Input {}

impl Input {
    pub fn just_pressed(&self, action: &str) -> bool {
        GodotInput::godot_singleton().is_action_just_pressed(action)
    }

    pub fn just_released(&self, action: &str) -> bool {
        GodotInput::godot_singleton().is_action_just_released(action)
    }

    pub fn pressed(&self, action: &str) -> bool {
        GodotInput::godot_singleton().is_action_pressed(action)
    }
}
