use bevy::prelude::Color;

pub const PLAY_AREA_WIDTH: f32 = 640.0;
pub const PLAY_AREA_HEIGHT: f32 = 480.0;

pub const PADDLE_HEIGHT: f32 = 12.0;
pub const PADDLE_WIDTH_RATIO: f32 = 5.0;
pub const PADDLE_DEFAULT_SPEED: f32 = 200.0;

pub const BALL_SIZE: f32 = 8.0;
pub const BALL_DEFAULT_SPEED: f32 = 200.0;

pub const BLOCKS_PER_ROW: i32 = 10;
pub const BLOCK_WIDTH: f32 = PLAY_AREA_WIDTH / BLOCKS_PER_ROW as f32;
pub const BLOCK_HEIGHT_RATIO: f32 = 1.0 / 4.0;

pub static ROW_COLORS: [Color; 8] = [
    Color::MAROON,
    Color::RED,
    Color::ORANGE,
    Color::YELLOW,
    Color::GREEN,
    Color::TEAL,
    Color::BLUE,
    Color::PURPLE,
];
