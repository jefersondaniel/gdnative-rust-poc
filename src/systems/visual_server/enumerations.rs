use bevy_ecs::schedule::{StageLabel};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum VisualServerStage {
    Remove,
    Update,
    Transform
}
