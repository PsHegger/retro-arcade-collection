use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::{Text, Text2dBundle, TextStyle};
use bevy::utils::default;

use crate::common::{AppState, Game, Renderable, ViewportSize};
use crate::constants::{FONT_FILE, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::scenes::menu::components::{MenuComponent, MenuGameItem};

pub struct MenuScenePlugin;

const GAMES_HORIZONTAL_MARGIN: f32 = 50.0;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(despawn_menu.in_schedule(OnExit(AppState::Menu)))
            .add_system(game_click_system.in_set(OnUpdate(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>, viewport_size: Res<ViewportSize>) {
    let font = assets.load(FONT_FILE.to_string());
    let title_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let target_resolution = Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let scale = viewport_size.height / WINDOW_HEIGHT;

    let title_pos = Vec2::new(0.0, WINDOW_HEIGHT / 2.0);
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Retro Arcade\nCollection", title_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(title_pos.x * scale, title_pos.y * scale, 0.0),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        MenuComponent,
        Renderable::new(title_pos, target_resolution).with_scale(false, false),
    ));

    let game_count = Game::supported_games().len();
    let row_1_count = if game_count <= 4 {
        game_count
    } else {
        if game_count % 2 == 0 {
            game_count / 2
        } else {
            game_count / 2 + 1
        }
    };
    let (game_size, trans_y) = if game_count <= row_1_count {
        (Vec2::new(270.0, 480.0), 0.0)
    } else {
        (Vec2::new(135.0, 240.0), 140.0)
    };

    for (i, game) in Game::supported_games().enumerate() {
        let is_first_row = i < row_1_count;
        let row_index = if is_first_row { i } else { i - row_1_count } as f32;
        let row_count = if is_first_row {
            row_1_count
        } else {
            game_count - row_1_count
        } as f32;
        let start_x = (-(row_count - 1.0)) * game_size.x / 2.0
            - (row_count - 1.0) * GAMES_HORIZONTAL_MARGIN / 2.0;
        let final_trans_x = start_x + row_index * (game_size.x + GAMES_HORIZONTAL_MARGIN);
        let final_trans_y = if is_first_row {
            trans_y
        } else {
            -1.0 * trans_y
        } - 50.0;
        let pos = Vec2::new(final_trans_x, final_trans_y);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: game.color(),
                    custom_size: Some(game_size),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x * scale, pos.y * scale, 0.0)
                    .with_scale(Vec3::new(scale, scale, 0.0)),
                ..default()
            },
            MenuComponent,
            MenuGameItem { game: game.clone() },
            Renderable::new(pos, target_resolution).with_size(game_size),
        ));
    }
}

pub fn despawn_menu(mut commands: Commands, components: Query<Entity, With<MenuComponent>>) {
    for component in components.iter() {
        commands.entity(component).despawn();
    }
}

pub fn game_click_system(
    mouse: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<AppState>>,
    windows: Query<&Window>,
    games: Query<(&MenuGameItem, &Sprite, &Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) else { return; };

    for (game, sprite, transform) in games.iter() {
        if let Some(bounds) = game_bounds(transform, sprite) {
            if bounds.contains(cursor_position) {
                match game.game {
                    Game::Asteroid => {}
                    Game::Bomberman => {}
                    Game::Breakout => next_state.set(AppState::Breakout),
                    Game::PacMan => {}
                    Game::Sokoban => {}
                    Game::SpaceInvaders => {}
                    Game::Tetris => {}
                    Game::Tron => {}
                };
            }
        }
    }
}

fn game_bounds(transform: &Transform, sprite: &Sprite) -> Option<Rect> {
    sprite.custom_size.map(|size| {
        let width = size.x * transform.scale.x;
        let height = size.y * transform.scale.y;
        Rect::new(
            transform.translation.x - width / 2.0,
            transform.translation.y - height / 2.0,
            transform.translation.x + width / 2.0,
            transform.translation.y + height / 2.0,
        )
    })
}
