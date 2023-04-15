use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::AppState;
use crate::constants::FONT_FILE;
use crate::scenes::breakout::components::*;
use crate::scenes::breakout::constants::*;
use crate::scenes::breakout::events::*;
use crate::scenes::breakout::logic::move_ball;
use crate::scenes::breakout::resources::{GameState, ViewportScale};

pub struct EventHandlerPlugin;

impl Plugin for EventHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            block_destroyed_event_handler
                .in_set(OnUpdate(AppState::Breakout))
                .after(move_ball),
        )
        .add_system(
            game_over_event_handler
                .in_set(OnUpdate(AppState::Breakout))
                .after(move_ball),
        )
        .add_system(play_sound_event_handler.in_set(OnUpdate(AppState::Breakout)))
        .add_system(restart_game_event_handler.in_set(OnUpdate(AppState::Breakout)));
    }
}

pub fn block_destroyed_event_handler(
    mut commands: Commands,
    mut events: EventReader<BlockDestroyedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for event in events.iter() {
        commands.entity(event.entity).despawn();
        game_state.score += event.block_value;
    }
}

pub fn play_sound_event_handler(
    mut events: EventReader<PlaySoundEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        let sound = match event.0 {
            SoundType::BallHitWall => asset_server.load("sound/breakout/ball_impact.ogg"),
            SoundType::BallHitPaddle => asset_server.load("sound/breakout/ball_paddle_impact.ogg"),
        };
        audio.play(sound);
    }
}

pub fn game_over_event_handler(
    mut commands: Commands,
    mut events: EventReader<GameOverEvent>,
    ball_query: Query<Entity, With<Ball>>,
    windows_query: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    let window = windows_query.single();
    if events.is_empty() {
        return;
    }
    events.clear();
    game_state.has_game_ended = true;

    for entity in ball_query.iter() {
        commands.entity(entity).despawn()
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 0.85),
                custom_size: Some(Vec2::new(window.width(), window.height())),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        EndGameUIElement,
        BreakoutEntity,
    ));

    let font = asset_server.load(FONT_FILE.to_string());
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Game Over",
                TextStyle {
                    font: font.clone(),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(0.0, 40.0, 11.0),
            text_anchor: Anchor::BottomCenter,
            ..default()
        },
        EndGameUIElement,
        BreakoutEntity,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Score: {}", game_state.score),
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(0.0, 40.0, 11.0),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        EndGameUIElement,
        BreakoutEntity,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "[Space]: New Game\n[Esc]: Menu",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, -window.height() / 2.0 + 10.0, 11.0),
            text_anchor: Anchor::BottomCenter,
            ..default()
        },
        EndGameUIElement,
        BreakoutEntity,
    ));
}

fn restart_game_event_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut restart_events: EventReader<RestartGameEvent>,
    entities_to_clear: Query<Entity, Or<(With<Renderable>, With<EndGameUIElement>)>>,
    mut game_state: ResMut<GameState>,
    viewport_scale: ResMut<ViewportScale>,
) {
    if restart_events.is_empty() {
        return;
    }
    restart_events.clear();

    game_state.reset();
    for entity in entities_to_clear.iter() {
        commands.entity(entity).despawn();
    }

    let scale = viewport_scale.0;
    let border_pos = PLAY_AREA_WIDTH / 2.0 + 1.0;
    let paddle_size = Vec2::new(PADDLE_WIDTH_RATIO * PADDLE_HEIGHT, PADDLE_HEIGHT);
    let paddle_pos = Vec2::new(0.0, -(PLAY_AREA_HEIGHT - PADDLE_HEIGHT) / 2.0);
    let ball_pos = Vec2::new(0.0, paddle_pos.y + (paddle_size.y + BALL_SIZE) / 2.0);
    // Spawn left border
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1.0, PLAY_AREA_HEIGHT * scale)),
                ..default()
            },
            transform: Transform::from_xyz(-border_pos * scale, 0.0, 1.0),
            ..default()
        },
        Renderable {
            pos: Vec2::new(-border_pos, 0.0),
            size: Vec2::new(1.0, PLAY_AREA_HEIGHT),
            scale_x: false,
            scale_y: true,
        },
        BreakoutEntity,
    ));
    // Spawn right border
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1.0, PLAY_AREA_HEIGHT * scale)),
                ..default()
            },
            transform: Transform::from_xyz(PLAY_AREA_WIDTH / 2.0 * scale, 0.0, 1.0),
            ..default()
        },
        Renderable {
            pos: Vec2::new(border_pos, 0.0),
            size: Vec2::new(1.0, PLAY_AREA_HEIGHT),
            scale_x: false,
            scale_y: true,
        },
        BreakoutEntity,
    ));
    // Spawn paddle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(paddle_size.clone() * scale),
                ..default()
            },
            texture: asset_server.load("sprites/breakout/paddle.png"),
            transform: Transform::from_xyz(paddle_pos.x * scale, paddle_pos.y * scale, 0.0),
            ..default()
        },
        Renderable {
            pos: paddle_pos,
            size: paddle_size,
            scale_x: true,
            scale_y: true,
        },
        Paddle {
            speed: PADDLE_DEFAULT_SPEED,
        },
        BreakoutEntity,
    ));
    // Spawn ball
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(BALL_SIZE * scale, BALL_SIZE * scale)),
                ..default()
            },
            texture: asset_server.load("sprites/breakout/ball.png"),
            transform: Transform::from_xyz(ball_pos.x * scale, ball_pos.y * scale, 1.0),
            ..default()
        },
        Renderable {
            pos: ball_pos,
            size: Vec2::new(BALL_SIZE, BALL_SIZE),
            scale_x: true,
            scale_y: true,
        },
        Ball::default(),
        BreakoutEntity,
    ));

    let row_count = ROW_SPRITES.iter().len() as i32;
    let block_size = Vec2::new(BLOCK_WIDTH, BLOCK_WIDTH * BLOCK_HEIGHT_RATIO);
    for (row, sprite_path) in ROW_SPRITES.iter().enumerate() {
        let pos_y = 200.0 - row as f32 * block_size.y;
        for i in 0..BLOCKS_PER_ROW {
            let pos = Vec2::new(
                -(PLAY_AREA_WIDTH - BLOCK_WIDTH) / 2.0 + i as f32 * block_size.x,
                pos_y,
            );
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(block_size * scale),
                        ..default()
                    },
                    texture: asset_server.load(sprite_path.to_string()),
                    transform: Transform::from_xyz(pos.x * scale, pos.y * scale, 0.0),
                    ..default()
                },
                Renderable {
                    pos,
                    size: block_size,
                    scale_x: true,
                    scale_y: true,
                },
                Block {
                    score: (row_count - row as i32) * 1000,
                },
                BreakoutEntity,
            ));
        }
    }

    // Spawn score text
    let font = asset_server.load(FONT_FILE.to_string());
    let score_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let score_pos_y = PLAY_AREA_HEIGHT / 2.0 - 5.0;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("0", score_style).with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, score_pos_y * scale, 2.0),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        Renderable {
            pos: Vec2::new(0.0, score_pos_y),
            size: Default::default(),
            scale_x: false,
            scale_y: false,
        },
        ScoreText,
        BreakoutEntity,
    ));
}
