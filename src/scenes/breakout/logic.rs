use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::time::Time;

use crate::common::AppState;
use crate::scenes::breakout::components::{Ball, Block, Paddle, Renderable, ScoreText};
use crate::scenes::breakout::constants::{
    SoundType, BALL_DEFAULT_SPEED, BALL_SPEED_INCREASE_SCORE, BALL_SPEED_INCREASE_VALUE,
};
use crate::scenes::breakout::event_handlers::block_destroyed_event_handler;
use crate::scenes::breakout::events::{BlockDestroyedEvent, GameOverEvent, PlaySoundEvent};
use crate::scenes::breakout::resources::GameState;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_ball_with_paddle.in_set(OnUpdate(AppState::Breakout)))
            .add_system(move_ball.in_set(OnUpdate(AppState::Breakout)))
            .add_system(
                score_change
                    .in_set(OnUpdate(AppState::Breakout))
                    .after(block_destroyed_event_handler),
            );
    }
}

pub fn move_ball_with_paddle(
    paddle_query: Query<&Renderable, (With<Paddle>, Changed<Renderable>)>,
    mut ball_query: Query<(&Ball, &mut Renderable), Without<Paddle>>,
) {
    let Ok(paddle_renderable) = paddle_query.get_single() else { return; };
    for (ball, mut renderable) in ball_query.iter_mut() {
        if ball.is_attached {
            renderable.pos.x = paddle_renderable.pos.x;
        }
    }
}

pub fn move_ball(
    time: Res<Time>,
    mut block_destroyed_events: EventWriter<BlockDestroyedEvent>,
    mut play_sound_events: EventWriter<PlaySoundEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
    mut ball_query: Query<(&mut Ball, &mut Renderable), (Without<Paddle>, Without<Block>)>,
    paddle_query: Query<&Renderable, (With<Paddle>, Without<Block>)>,
    blocks_query: Query<(Entity, &Block, &Renderable)>,
) {
    for (mut ball, mut ball_renderable) in ball_query.iter_mut() {
        if blocks_query.is_empty() {
            // cleared all blocks: win
            game_over_events.send_default();
            return;
        }
        if !ball.is_attached {
            let mut sound: Option<SoundType> = None;
            let mut new_pos = ball_renderable.pos + ball.dir * ball.speed * time.delta_seconds();
            if new_pos.x < ball_renderable.min_x() {
                // Collision with left wall
                new_pos.x = ball_renderable.min_x();
                ball.dir.x = -ball.dir.x;
                sound = Some(SoundType::BallHitWall);
            } else if new_pos.x > ball_renderable.max_x() {
                // Collision with right wall
                new_pos.x = ball_renderable.max_x();
                ball.dir.x = -ball.dir.x;
                sound = Some(SoundType::BallHitWall);
            }
            if new_pos.y > ball_renderable.max_y() {
                // Collision with top
                new_pos.y = ball_renderable.max_y();
                ball.dir.y = -ball.dir.y;
                sound = Some(SoundType::BallHitWall);
            } else if new_pos.y < ball_renderable.min_y() {
                // Ball reached bottom
                game_over_events.send_default();
            }

            // Check for collision with paddle
            let Ok(paddle) = paddle_query.get_single() else { continue; };
            if new_pos.x >= paddle.left()
                && new_pos.x <= paddle.right()
                && new_pos.y <= paddle.top() + ball_renderable.size.y / 2.0
            {
                new_pos.y = paddle.top() + ball_renderable.size.y / 2.0;
                ball.dir =
                    Vec2::new((new_pos.x - paddle.pos.x) / (paddle.size.x / 2.0), 1.0).normalize();
                sound = Some(SoundType::BallHitPaddle);
            }

            // check for collision with any block
            for (entity, block, block_renderable) in blocks_query.iter() {
                let mut block_hit = false;

                if new_pos.x >= block_renderable.left() && new_pos.x <= block_renderable.right() {
                    // check if hit from below or top
                    let ball_top = new_pos.y + ball_renderable.size.y / 2.0;
                    let ball_bottom = new_pos.y - ball_renderable.size.y / 2.0;

                    if ball_top >= block_renderable.bottom()
                        && ball_bottom < block_renderable.bottom()
                    {
                        block_hit = true;
                    } else if ball_bottom <= block_renderable.top()
                        && ball_top > block_renderable.top()
                    {
                        block_hit = true;
                    }
                    if block_hit {
                        ball.dir.y = -ball.dir.y;
                    }
                } else if new_pos.y >= block_renderable.bottom()
                    && new_pos.y <= block_renderable.top()
                {
                    // check if hit from left or right
                    let ball_left = new_pos.x - ball_renderable.size.x / 2.0;
                    let ball_right = new_pos.x + ball_renderable.size.x / 2.0;

                    if ball_right >= block_renderable.left() && ball_left < block_renderable.left()
                    {
                        block_hit = true;
                    } else if ball_left <= block_renderable.right()
                        && ball_right > block_renderable.right()
                    {
                        block_hit = true;
                    }
                    if block_hit {
                        ball.dir.x = -ball.dir.x;
                    }
                }

                if block_hit {
                    block_destroyed_events.send(BlockDestroyedEvent {
                        entity,
                        block_value: block.score,
                    });
                    sound = Some(SoundType::BallHitWall);
                }
            }

            ball_renderable.pos = new_pos;
            if let Some(sound_type) = sound {
                play_sound_events.send(PlaySoundEvent(sound_type));
            }
        }
    }
}

pub fn score_change(
    game_state: Res<GameState>,
    mut ball_query: Query<&mut Ball>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
) {
    if !game_state.is_changed() {
        return;
    }
    for mut ball in ball_query.iter_mut() {
        ball.speed = BALL_DEFAULT_SPEED
            + (game_state.score / BALL_SPEED_INCREASE_SCORE) as f32 * BALL_SPEED_INCREASE_VALUE;
    }
    for mut score_label in score_query.iter_mut() {
        score_label.sections[0].value = format!("{}", game_state.score);
    }
}
