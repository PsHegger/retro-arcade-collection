use bevy::app::App;
use bevy::prelude::*;

use crate::common::ViewportSize;
use crate::scenes::breakout::components::{Paddle, Renderable};
use crate::scenes::breakout::constants::*;
use crate::scenes::breakout::resources::ViewportScale;

pub struct BreakoutScenePlugin;

impl Plugin for BreakoutScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ViewportScale::default())
            .add_startup_system(foo)
            .add_system(update_scale)
            .add_system(keyboard_input.before(renderable_translate_handler))
            .add_system(renderable_translate_handler);
    }
}

fn foo(mut commands: Commands) {
    let border_pos = PLAY_AREA_WIDTH / 2.0;
    let paddle_size = Vec2::new(PADDLE_WIDTH_RATIO * PADDLE_HEIGHT, PADDLE_HEIGHT);
    let paddle_pos = Vec2::new(0.0, -(PLAY_AREA_HEIGHT - PADDLE_HEIGHT) / 2.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1.0, PLAY_AREA_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(-border_pos, 0.0, 0.0),
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
            transform: Transform::from_xyz(PLAY_AREA_WIDTH / 2.0, 0.0, 0.0),
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
}

fn update_scale(
    viewport_size: Res<ViewportSize>,
    mut viewport_scale: ResMut<ViewportScale>,
    mut query: Query<(&mut Sprite, &mut Transform, &Renderable)>,
) {
    if !viewport_size.is_changed() { return; }
    let scale = viewport_size.height / PLAY_AREA_HEIGHT;

    viewport_scale.scale = scale;

    for (mut sprite, mut transform, renderable) in query.iter_mut() {
        let scaled_width = if renderable.scale_x { renderable.size.x * scale } else { renderable.size.x };
        let scaled_height = if renderable.scale_y { renderable.size.y * scale } else { renderable.size.y };
        let scaled_pos_x = renderable.pos.x * scale;
        let scaled_pos_y = renderable.pos.y * scale;
        sprite.custom_size = Some(Vec2::new(scaled_width, scaled_height));
        transform.translation = Vec3::new(scaled_pos_x, scaled_pos_y, 0.0);
    }
}

fn renderable_translate_handler(
    viewport_scale: Res<ViewportScale>,
    mut query: Query<(&mut Transform, &Renderable), Changed<Renderable>>,
) {
    for (mut transform, renderable) in query.iter_mut() {
        let scaled_pos_x = renderable.pos.x * viewport_scale.scale;
        let scaled_pos_y = renderable.pos.y * viewport_scale.scale;
        transform.translation = Vec3::new(scaled_pos_x, scaled_pos_y, 0.0);
    }
}

fn keyboard_input(
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
