use bevy::prelude::Entity;

pub struct BlockDestroyedEvent {
    pub entity: Entity,
    pub block_value: i32,
}
