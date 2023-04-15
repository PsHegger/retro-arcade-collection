use bevy::math::Vec2;
use bevy::prelude::Component;

use crate::scenes::breakout::constants::BALL_DEFAULT_SPEED;

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
}

#[derive(Component)]
pub struct Ball {
    pub is_attached: bool,
    pub dir: Vec2,
    pub speed: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            is_attached: true,
            dir: Vec2::ZERO,
            speed: BALL_DEFAULT_SPEED,
        }
    }
}

#[derive(Component)]
pub struct Block {
    pub score: i32,
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct EndGameUIElement;

#[derive(Component)]
pub struct BreakoutEntity;
