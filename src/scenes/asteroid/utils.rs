use bevy::prelude::SystemSet;

#[derive(SystemSet, Debug, Hash, Eq, PartialEq, Clone)]
pub enum FrameSet {
    Input,
    GameLogic,
    EventHandling,
    Rendering,
}
