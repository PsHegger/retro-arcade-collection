use bevy::prelude::*;

use crate::scenes::breakout::constants::SoundType;
use crate::scenes::breakout::events::{BlockDestroyedEvent, PlaySoundEvent};
use crate::scenes::breakout::resources::PlayerScore;

pub fn block_destroyed_event_handler(
    mut commands: Commands,
    mut events: EventReader<BlockDestroyedEvent>,
    mut player_score: ResMut<PlayerScore>,
) {
    for event in events.iter() {
        commands.entity(event.entity).despawn();
        player_score.0 += event.block_value;
    }
}

pub fn play_sound_event_handler(
    mut events: EventReader<PlaySoundEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        let sound = match event.0 {
            SoundType::BallHitWall => asset_server.load("sound/breakout/ball_impact.ogg"),
            SoundType::BallHitPaddle => asset_server.load("sound/breakout/ball_paddle_impact.ogg"),
        };
        audio.play(sound);
    }
}
