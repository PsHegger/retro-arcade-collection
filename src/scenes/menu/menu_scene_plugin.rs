use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::{Text, Text2dBundle, TextStyle};
use bevy::utils::default;

use crate::common::Game;
use crate::constants::WINDOW_HEIGHT;
use crate::scenes::menu::components::{MenuComponent, MenuGameItem};

pub struct MenuScenePlugin;

const GAMES_HORIZONTAL_MARGIN: f32 = 50.0;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("RobotoMono.ttf");
    let title_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Retro Arcade\nCollection", title_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0., WINDOW_HEIGHT / 2.0, 0.0),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        MenuComponent
    ));

    let game_count = Game::supported_games().len();
    let row_1_count = if game_count <= 4 { game_count } else {
        if game_count % 2 == 0 { game_count / 2 } else { game_count / 2 + 1}
    };
    let (game_size, trans_y) = if game_count <= row_1_count {
        (Vec2::new(270.0, 480.0), 0.0)
    } else {
        (Vec2::new(135.0, 240.0), 140.0)
    };

    for (i, game) in Game::supported_games().enumerate() {
        let is_first_row = i < row_1_count;
        let row_index = if is_first_row { i } else { i - row_1_count } as f32;
        let row_count = if is_first_row { row_1_count } else { game_count - row_1_count } as f32;
        let start_x = (-(row_count - 1.0)) * game_size.x / 2.0 - (row_count - 1.0) * GAMES_HORIZONTAL_MARGIN / 2.0;
        let final_trans_x = start_x + row_index * (game_size.x + GAMES_HORIZONTAL_MARGIN);
        let final_trans_y = if is_first_row { trans_y } else { -1.0 * trans_y } - 50.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: game.color(),
                    custom_size: Some(game_size),
                    ..default()
                },
                transform: Transform::from_xyz(final_trans_x, final_trans_y, 0.0),
                ..default()
            },
            MenuComponent,
            MenuGameItem {
                game: game.clone(),
            }
        ));
    }
}
