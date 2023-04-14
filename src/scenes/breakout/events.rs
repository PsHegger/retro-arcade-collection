use bevy::prelude::Entity;

use crate::scenes::breakout::constants::SoundType;

pub struct BlockDestroyedEvent {
    pub entity: Entity,
    pub block_value: i32,
}

pub struct PlaySoundEvent(pub SoundType);
