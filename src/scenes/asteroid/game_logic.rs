use bevy::prelude::*;

use crate::common::AppState;
use crate::scenes::asteroid::components::Ship;
use crate::scenes::asteroid::utils::FrameSet;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            move_ship_system
                .in_set(OnUpdate(AppState::Asteroid))
                .in_set(FrameSet::GameLogic),
        );
    }
}

fn move_ship_system(time: Res<Time>, mut ship_q: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in ship_q.iter_mut() {
        transform.translation.x += ship.speed.x * time.delta_seconds();
        transform.translation.y += ship.speed.y * time.delta_seconds();

        // TODO: handle warping back when leaving play area
    }
}
