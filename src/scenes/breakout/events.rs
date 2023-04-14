use bevy::app::{App, Plugin};
use bevy::prelude::Entity;

use crate::scenes::breakout::constants::SoundType;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockDestroyedEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<PlaySoundEvent>()
            .add_event::<RestartGameEvent>();
    }
}

pub struct BlockDestroyedEvent {
    pub entity: Entity,
    pub block_value: i32,
}

pub struct PlaySoundEvent(pub SoundType);

#[derive(Default)]
pub struct GameOverEvent;

#[derive(Default)]
pub struct RestartGameEvent;
