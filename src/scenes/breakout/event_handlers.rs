use bevy::prelude::*;

use crate::scenes::breakout::events::BlockDestroyedEvent;
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
