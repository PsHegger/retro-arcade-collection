use bevy::app::App;
use bevy::prelude::*;

use crate::scenes::breakout::components::{Ball, Block, Paddle, Renderable};
use crate::scenes::breakout::constants::*;
use crate::scenes::breakout::input::*;
use crate::scenes::breakout::logic::*;
use crate::scenes::breakout::rendering::*;
use crate::scenes::breakout::resources::ViewportScale;

pub struct BreakoutScenePlugin;

impl Plugin for BreakoutScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ViewportScale::default())
            .add_startup_system(setup_scene)
            .add_system(scale_handler)
            .add_system(paddle_keyboard_input)
            .add_system(move_ball_with_paddle)
            .add_system(renderable_transform_handler.after(move_ball_with_paddle));
    }
}

fn setup_scene(mut commands: Commands) {
    let border_pos = PLAY_AREA_WIDTH / 2.0;
    let paddle_size = Vec2::new(PADDLE_WIDTH_RATIO * PADDLE_HEIGHT, PADDLE_HEIGHT);
    let paddle_pos = Vec2::new(0.0, -(PLAY_AREA_HEIGHT - PADDLE_HEIGHT) / 2.0);
    let ball_pos = Vec2::new(0.0, paddle_pos.y + (paddle_size.y + BALL_SIZE) / 2.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1.0, PLAY_AREA_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(-border_pos, 0.0, 1.0),
            ..default()
        },
        Renderable {
            pos: Vec2::new(-border_pos, 0.0),
            size: Vec2::new(1.0, PLAY_AREA_HEIGHT),
            scale_x: false,
            scale_y: true,
        }
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1.0, PLAY_AREA_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(PLAY_AREA_WIDTH / 2.0, 0.0, 1.0),
            ..default()
        },
        Renderable {
            pos: Vec2::new(border_pos, 0.0),
            size: Vec2::new(1.0, PLAY_AREA_HEIGHT),
            scale_x: false,
            scale_y: true,
        }
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(paddle_size.clone()),
                ..default()
            },
            transform: Transform::from_xyz(paddle_pos.x, paddle_pos.y, 0.0),
            ..default()
        },
        Renderable {
            pos: paddle_pos,
            size: paddle_size,
            scale_x: true,
            scale_y: true,
        },
        Paddle { speed: PADDLE_DEFAULT_SPEED },
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(ball_pos.x, ball_pos.y, 0.0),
            ..default()
        },
        Renderable {
            pos: ball_pos,
            size: Vec2::new(BALL_SIZE, BALL_SIZE),
            scale_x: true,
            scale_y: true,
        },
        Ball { is_attached: true },
    ));

    let row_count = ROW_COLORS.iter().len() as i32;
    let block_size = Vec2::new(BLOCK_WIDTH, BLOCK_WIDTH * BLOCK_HEIGHT_RATIO);
    for (row, color) in ROW_COLORS.iter().enumerate() {
        let pos_y = 200.0 - row as f32 * block_size.y;
        for i in 0..BLOCKS_PER_ROW {
            let pos = Vec2::new(-(PLAY_AREA_WIDTH - BLOCK_WIDTH) / 2.0 + i as f32 * block_size.x, pos_y);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: color.clone(),
                        custom_size: Some(block_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    ..default()
                },
                Renderable {
                    pos,
                    size: block_size,
                    scale_x: true,
                    scale_y: true,
                },
                Block { score: (row_count - i as i32) * 1000 },
            ));
        }
    }
}


