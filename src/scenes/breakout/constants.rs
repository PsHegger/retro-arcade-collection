pub const PLAY_AREA_WIDTH: f32 = 640.0;
pub const PLAY_AREA_HEIGHT: f32 = 480.0;

pub const PADDLE_HEIGHT: f32 = 12.0;
pub const PADDLE_WIDTH_RATIO: f32 = 5.0;
pub const PADDLE_DEFAULT_SPEED: f32 = 200.0;

pub const BALL_SIZE: f32 = 8.0;
pub const BALL_DEFAULT_SPEED: f32 = 200.0;
pub const BALL_SPEED_INCREASE_SCORE: i32 = 5000;
pub const BALL_SPEED_INCREASE_VALUE: f32 = 10.0;

pub const BLOCKS_PER_ROW: i32 = 10;
pub const BLOCK_WIDTH: f32 = PLAY_AREA_WIDTH / BLOCKS_PER_ROW as f32;
pub const BLOCK_HEIGHT_RATIO: f32 = 1.0 / 4.0;

pub static ROW_SPRITES: [&str; 8] = [
    "sprites/breakout/block_maroon.png",
    "sprites/breakout/block_red.png",
    "sprites/breakout/block_orange.png",
    "sprites/breakout/block_yellow.png",
    "sprites/breakout/block_green.png",
    "sprites/breakout/block_teal.png",
    "sprites/breakout/block_blue.png",
    "sprites/breakout/block_purple.png",
];

pub enum SoundType {
    BallHitWall,
    BallHitPaddle,
}
