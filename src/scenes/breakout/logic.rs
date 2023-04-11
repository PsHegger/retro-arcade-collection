use bevy::prelude::{Changed, Query, With, Without};

use crate::scenes::breakout::components::{Ball, Paddle, Renderable};

pub fn move_ball_with_paddle(
    paddle_query: Query<&Renderable, (With<Paddle>, Changed<Renderable>)>,
    mut ball_query: Query<(&Ball, &mut Renderable), Without<Paddle>>,
) {
    if let Some(paddle_renderable) = paddle_query.iter().nth(0) {
        for (ball, mut renderable) in ball_query.iter_mut() {
            if ball.is_attached {
                renderable.pos.x = paddle_renderable.pos.x;
            }
        }
    }
}
