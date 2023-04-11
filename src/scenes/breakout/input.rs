use bevy::prelude::*;

use crate::scenes::breakout::components::{Paddle, Renderable};
use crate::scenes::breakout::constants::PLAY_AREA_WIDTH;

pub fn paddle_keyboard_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Renderable, &Paddle)>,
) {
    for (mut renderable, paddle) in query.iter_mut() {
        if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
            let mut new_pos = renderable.pos.x - time.delta_seconds() * paddle.speed;
            if new_pos < -(PLAY_AREA_WIDTH - renderable.size.x) / 2.0 {
                new_pos = -(PLAY_AREA_WIDTH - renderable.size.x) / 2.0;
            }
            renderable.pos.x = new_pos;
        } else if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
            let mut new_pos = renderable.pos.x + time.delta_seconds() * paddle.speed;
            if new_pos > (PLAY_AREA_WIDTH - renderable.size.x) / 2.0 {
                new_pos = (PLAY_AREA_WIDTH - renderable.size.x) / 2.0;
            }
            renderable.pos.x = new_pos;
        }
    }
}
