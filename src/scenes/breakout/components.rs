use bevy::math::Vec2;
use bevy::prelude::Component;

use crate::scenes::breakout::constants::{BALL_DEFAULT_SPEED, PLAY_AREA_HEIGHT, PLAY_AREA_WIDTH};

#[derive(Component, Debug)]
pub struct Renderable {
    pub pos: Vec2,
    pub size: Vec2,
    pub scale_x: bool,
    pub scale_y: bool,
}

impl Renderable {
    pub fn left(&self) -> f32 {
        self.pos.x - self.size.x / 2.0
    }

    pub fn right(&self) -> f32 {
        self.pos.x + self.size.x / 2.0
    }

    pub fn top(&self) -> f32 {
        self.pos.y + self.size.y / 2.0
    }

    pub fn bottom(&self) -> f32 {
        self.pos.y - self.size.y / 2.0
    }

    pub fn min_x(&self) -> f32 {
        -(PLAY_AREA_WIDTH - self.size.x) / 2.0
    }

    pub fn max_x(&self) -> f32 {
        (PLAY_AREA_WIDTH - self.size.x) / 2.0
    }

    pub fn min_y(&self) -> f32 {
        -(PLAY_AREA_HEIGHT - self.size.y) / 2.0
    }

    pub fn max_y(&self) -> f32 {
        (PLAY_AREA_HEIGHT - self.size.y) / 2.0
    }
}

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
