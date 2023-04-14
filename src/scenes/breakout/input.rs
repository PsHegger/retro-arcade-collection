use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::scenes::breakout::components::{Ball, Paddle, Renderable};
use crate::scenes::breakout::events::RestartGameEvent;
use crate::scenes::breakout::resources::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(paddle_keyboard_input)
            .add_system(ball_keyboard_input)
            .add_system(end_game_keyboard_input);
    }
}

pub fn paddle_keyboard_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    game_state: Res<GameState>,
    mut query: Query<(&mut Renderable, &Paddle)>,
) {
    if game_state.has_game_ended {
        return;
    }

    for (mut renderable, paddle) in query.iter_mut() {
        if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
            let mut new_pos = renderable.pos.x - time.delta_seconds() * paddle.speed;
            if new_pos < renderable.min_x() {
                new_pos = renderable.min_x();
            }
            renderable.pos.x = new_pos;
        } else if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
            let mut new_pos = renderable.pos.x + time.delta_seconds() * paddle.speed;
            if new_pos > renderable.max_x() {
                new_pos = renderable.max_x();
            }
            renderable.pos.x = new_pos;
        }
    }
}

pub fn ball_keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Ball>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut ball in query.iter_mut() {
            if ball.is_attached {
                ball.is_attached = false;
                ball.dir = Vec2::new(thread_rng().gen_range(-1.0..1.0), 1.0).normalize()
            }
        }
    }
}

pub fn end_game_keyboard_input(
    keys: Res<Input<KeyCode>>,
    game_state: Res<GameState>,
    mut restart_events: EventWriter<RestartGameEvent>,
) {
    if !game_state.has_game_ended {
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        restart_events.send_default();
    }
}
