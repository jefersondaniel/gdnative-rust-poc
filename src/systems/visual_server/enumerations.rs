use bevy_ecs::schedule::{StageLabel, SystemLabel};

#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualServerLabel {
    Remove,
    Update,
    Transform
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum VisualServerStage {
    Update,
}
